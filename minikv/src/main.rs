mod comandos;
mod errores;
mod almacenamiento;

use std::env;
use std::collections::HashMap;
use comandos::Comandos;
use errores::MiniKVError;
use almacenamiento::{cargar_estado, escribir_log, escribir_snapshot};

fn parsear_comandos(args: Vec<String>) -> Result<Comandos, MiniKVError> {
    match args.get(1).map(|s| s.as_str()) {
        Some("set") => {
            let key = args.get(2).ok_or(MiniKVError::FaltaKey)?;
            match args.get(3) {
                Some(value) => Ok(Comandos::Set { key: key.to_owned(), value: Some(value.to_owned()) }),
                None        => Ok(Comandos::Set { key: key.to_owned(), value: None }),
            }
        }
        Some("get") => {
            let key = args.get(2).ok_or(MiniKVError::FaltaKey)?;
            Ok(Comandos::Get { key: key.to_owned() })
        }
        Some("length")   => Ok(Comandos::Length),
        Some("snapshot") => Ok(Comandos::Snapshot),
        _                => Err(MiniKVError::ComandoInvalido),
    }
}



fn ejecutar_comando(comando: Comandos, store: &mut HashMap<String, Option<String>>) -> Result<(), MiniKVError> {
    match comando {
        Comandos::Set { key, value } => {
            escribir_log(&key, &value)?;
            store.insert(key, value);
            println!("OK");
        }
        Comandos::Get { key } => {
            match store.get(&key) {
                Some(Some(val)) => println!("{}", val),
                _               => println!("NOT FOUND"),             //decido no parar la ejecución del programa porque no es un error recuperable
            }
        }
        Comandos::Length => {
            println!("{}", store.values().filter(|v| v.is_some()).count());
        }
        Comandos::Snapshot => {
            escribir_snapshot(store)?;
            println!("OK");
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut store: HashMap<String, Option<String>> = HashMap::new();

    cargar_estado(&mut store);

    match parsear_comandos(args) {
        Ok(cmd) => {
            if let Err(e) = ejecutar_comando(cmd, &mut store) {
                println!("Error: {}", e);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}




//------------------------------------TEST------------------------------------//

#[cfg(test)]
mod tests {
    use super::*;

    fn args(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_set_con_valor() {
        let resultado = parsear_comandos(args(&["minikv", "set", "clave1", "valor1"]));
        match resultado {
            Ok(Comandos::Set { key, value: Some(v) }) => {
                assert_eq!(key, "clave1");
                assert_eq!(v, "valor1");
            }
            _ => panic!("Se esperaba Set con valor"),
        }
    }

    #[test]
    fn test_set_sin_valor() {
        let resultado = parsear_comandos(args(&["minikv", "set", "clave1"]));
        match resultado {
            Ok(Comandos::Set { key, value: None }) => assert_eq!(key, "clave1"),
            _ => panic!("Se esperaba Set sin valor"),
        }
    }

    #[test]
    fn test_set_sin_key() {
        let resultado = parsear_comandos(args(&["minikv", "set"]));
        assert!(matches!(resultado, Err(MiniKVError::FaltaKey)));
    }

    #[test]
    fn test_get() {
        let resultado = parsear_comandos(args(&["minikv", "get", "clave1"]));
        match resultado {
            Ok(Comandos::Get { key }) => assert_eq!(key, "clave1"),
            _ => panic!("Se esperaba Get"),
        }
    }

    #[test]
    fn test_get_sin_key() {
        let resultado = parsear_comandos(args(&["minikv", "get"]));
        assert!(matches!(resultado, Err(MiniKVError::FaltaKey)));
    }

    #[test]
    fn test_length() {
        assert!(matches!(parsear_comandos(args(&["minikv", "length"])), Ok(Comandos::Length)));
    }

    #[test]
    fn test_snapshot() {
        assert!(matches!(parsear_comandos(args(&["minikv", "snapshot"])), Ok(Comandos::Snapshot)));
    }

    #[test]
    fn test_comando_invalido() {
        assert!(matches!(parsear_comandos(args(&["minikv", "delete"])), Err(MiniKVError::ComandoInvalido)));
    }
}