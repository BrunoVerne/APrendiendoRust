use crate::ejecucion_comandos::ejecutar_comando;
use crate::errores::MiniKVError;
use crate::parseo_comandos::parsear_comandos;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

/// Maneja una conexión TCP entrante, ejecutando los comandos recibidos
/// y enviando las respuestas al cliente.
pub fn handle_connection(stream: TcpStream, store: Arc<Mutex<HashMap<String, Option<String>>>>) {
    let writer = match stream.try_clone() {
        Ok(s) => s,
        Err(e) => {
            println!("{}", MiniKVError::ErrorIO(e.to_string()));
            return;
        }
    };
    let reader = BufReader::new(stream);
    handle_lines(reader, writer, store);
}

fn parsear_tokens(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else if c == '"' {
            tokens.push(parsear_token_quoted(&mut chars));
        } else {
            tokens.push(parsear_token_plain(&mut chars));
        }
    }
    tokens
}

fn parsear_token_quoted(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
    chars.next(); // consumir "
    let mut token = String::new();
    while let Some(&ch) = chars.peek() {
        chars.next();
        if ch == '\\' {
            if let Some(&next) = chars.peek() {
                chars.next();
                token.push(next);
            }
        } else if ch == '"' {
            break;
        } else {
            token.push(ch);
        }
    }
    token
}

fn parsear_token_plain(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let mut token = String::new();
    while let Some(&ch) = chars.peek() {
        if ch.is_whitespace() {
            break;
        }
        chars.next();
        token.push(ch);
    }
    token
}

fn handle_lines<R, W>(
    reader: BufReader<R>,
    mut writer: W,
    store: Arc<Mutex<HashMap<String, Option<String>>>>,
) where
    R: std::io::Read,
    W: Write,
{
    for line in reader.lines() {
        let Ok(line) = line else {
            println!("{}", MiniKVError::ConnectionClosed);
            return;
        };

        let mut args = vec!["minikv-server".to_string()];
        args.extend(parsear_tokens(&line));

        let mut store = match store.lock() {
            Ok(s) => s,
            Err(e) => {
                println!("{}", MiniKVError::ErrorIO(e.to_string()));
                return;
            }
        };

        match parsear_comandos(args) {
            Ok(cmd) => {
                if let Err(e) = ejecutar_comando(cmd, &mut store, &mut writer) {
                    writeln!(writer, "{}", e).ok();
                }
            }
            Err(e) => {
                writeln!(writer, "{}", e).ok();
            }
        }
    }
}

//-----------------------TEST UNITARIOS-------------------------------//

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn store_vacio() -> Arc<Mutex<HashMap<String, Option<String>>>> {
        Arc::new(Mutex::new(HashMap::new()))
    }

    fn ejecutar(input: &str, store: Arc<Mutex<HashMap<String, Option<String>>>>) -> String {
        let reader = BufReader::new(Cursor::new(input.to_string()));
        let mut output = Vec::new();
        handle_lines(reader, &mut output, store);
        String::from_utf8(output).unwrap()
    }

    #[test]
    fn test_set_responde_ok() {
        let store = store_vacio();
        let salida = ejecutar("set clave valor\n", store);
        assert_eq!(salida, "OK\n");
    }

    #[test]
    fn test_get_existente() {
        let store = store_vacio();
        ejecutar("set clave valor\n", Arc::clone(&store));
        let salida = ejecutar("get clave\n", Arc::clone(&store));
        assert_eq!(salida, "valor\n");
    }

    #[test]
    fn test_length_vacio() {
        let store = store_vacio();
        let salida = ejecutar("length\n", store);
        assert_eq!(salida, "0\n");
    }

    #[test]
    fn test_length_con_elementos() {
        let store = store_vacio();
        ejecutar("set a b\nset c d\n", Arc::clone(&store));
        let salida = ejecutar("length\n", Arc::clone(&store));
        assert_eq!(salida, "2\n");
    }

    #[test]
    fn test_multiples_comandos_secuenciales() {
        let store = store_vacio();
        let salida = ejecutar("set a 1\nset b 2\nget a\nget b\nlength\n", store);
        assert_eq!(salida, "OK\nOK\n1\n2\n2\n");
    }

    #[test]
    fn test_set_sin_valor() {
        let store = store_vacio();
        let salida = ejecutar("set clave\n", store);
        assert_eq!(salida, "OK\n");
    }

    #[test]
    fn test_set_sin_valor_no_cuenta_en_length() {
        let store = store_vacio();
        ejecutar("set a\nset b valor\n", Arc::clone(&store));
        let salida = ejecutar("length\n", Arc::clone(&store));
        assert_eq!(salida, "1\n");
    }

    #[test]
    fn test_store_compartido_entre_llamadas() {
        let store = store_vacio();
        ejecutar("set clave valor\n", Arc::clone(&store));
        // simulamos segundo cliente con mismo store
        let salida = ejecutar("get clave\n", Arc::clone(&store));
        assert_eq!(salida, "valor\n");
    }
}
