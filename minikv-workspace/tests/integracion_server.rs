use minikv::handle_connection::handle_connection;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn levantar_server(addr: &str) {
    let listener = TcpListener::bind(addr).unwrap();
    let store: Arc<Mutex<HashMap<String, Option<String>>>> = Arc::new(Mutex::new(HashMap::new()));

    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(s) => {
                    let store = Arc::clone(&store);
                    thread::spawn(move || handle_connection(s, store));
                }
                Err(_) => break,
            }
        }
    });

    thread::sleep(Duration::from_millis(100));
}

fn enviar(addr: &str, comando: &str) -> String {
    let mut stream = TcpStream::connect(addr).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();
    writeln!(stream, "{}", comando).unwrap();
    let mut reader = BufReader::new(stream);
    let mut respuesta = String::new();
    reader.read_line(&mut respuesta).unwrap();
    respuesta.trim().to_string()
}

fn enviar_multiples(addr: &str, comandos: &[&str]) -> Vec<String> {
    let mut stream = TcpStream::connect(addr).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut resultados = Vec::new();

    for cmd in comandos {
        writeln!(stream, "{}", cmd).unwrap();
        let mut linea = String::new();
        reader.read_line(&mut linea).unwrap();
        resultados.push(linea.trim().to_string());
    }

    resultados
}

// Cada test usa un puerto distinto para no pisarse entre sí

#[test]
fn test_set_y_get_via_tcp() {
    levantar_server("127.0.0.1:18001");
    assert_eq!(enviar("127.0.0.1:18001", "set x 42"), "OK");
    assert_eq!(enviar("127.0.0.1:18001", "get x"), "42");
}

#[test]
fn test_not_found_via_tcp() {
    levantar_server("127.0.0.1:18002");
    assert_eq!(
        enviar("127.0.0.1:18002", "get noexiste"),
        "ERROR \"NOT FOUND\""
    );
}

#[test]
fn test_extra_argument_via_tcp() {
    levantar_server("127.0.0.1:18003");
    assert_eq!(
        enviar("127.0.0.1:18003", "set a b c"),
        "ERROR \"EXTRA ARGUMENT\""
    );
}

#[test]
fn test_unknown_command_via_tcp() {
    levantar_server("127.0.0.1:18004");
    assert_eq!(
        enviar("127.0.0.1:18004", "delete a"),
        "ERROR \"UNKNOWN COMMAND\""
    );
}

#[test]
fn test_length_via_tcp() {
    levantar_server("127.0.0.1:18005");
    enviar("127.0.0.1:18005", "set a 1");
    enviar("127.0.0.1:18005", "set b 2");
    assert_eq!(enviar("127.0.0.1:18005", "length"), "2");
}

#[test]
fn test_multiples_comandos_misma_conexion() {
    levantar_server("127.0.0.1:18006");
    let respuestas = enviar_multiples(
        "127.0.0.1:18006",
        &["set a 1", "set b 2", "get a", "get b", "length"],
    );
    assert_eq!(respuestas, vec!["OK", "OK", "1", "2", "2"]);
}

#[test]
fn test_dos_clientes_comparten_store() {
    levantar_server("127.0.0.1:18007");
    // cliente 1 escribe
    enviar("127.0.0.1:18007", "set compartida hola");
    // cliente 2 lee
    assert_eq!(enviar("127.0.0.1:18007", "get compartida"), "hola");
}

#[test]
fn test_cliente_no_puede_conectar() {
    // puerto que nadie escucha
    let result = TcpStream::connect("127.0.0.1:19999");
    assert!(result.is_err());
}

#[test]
fn test_conexion_se_cierra_limpiamente() {
    levantar_server("127.0.0.1:18008");
    // el server no debe caerse cuando el cliente cierra la conexion
    {
        let _stream = TcpStream::connect("127.0.0.1:18008").unwrap();
        // stream se dropea acá, cerrando la conexión
    }
    // el server sigue vivo — podemos conectarnos de nuevo
    assert_eq!(enviar("127.0.0.1:18008", "length"), "0");
}

#[test]
fn test_sobreescritura_valor() {
    levantar_server("127.0.0.1:18009");
    enviar("127.0.0.1:18009", "set k v1");
    enviar("127.0.0.1:18009", "set k v2");
    assert_eq!(enviar("127.0.0.1:18009", "get k"), "v2");
}
