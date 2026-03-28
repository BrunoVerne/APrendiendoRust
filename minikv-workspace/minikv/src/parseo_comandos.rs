use crate::comandos::Comandos;
use crate::errores::MiniKVError;

pub fn parsear_comandos(args: Vec<String>) -> Result<Comandos, MiniKVError> {
    match args.get(1).map(|s| s.as_str()) {
        Some("set") => {
            let key = args.get(2).ok_or(MiniKVError::MissingArgument)?;
            match args.get(3) {
                Some(value) => {
                    if args.get(4).is_some() {
                        return Err(MiniKVError::ExtraArgument);
                    }
                    Ok(Comandos::Set {
                        key: key.to_owned(),
                        value: Some(value.to_owned()),
                    })
                }
                None => Ok(Comandos::Set {
                    key: key.to_owned(),
                    value: None,
                }),
            }
        }
        Some("get") => {
            let key = args.get(2).ok_or(MiniKVError::MissingArgument)?;
            if args.get(3).is_some() {
                return Err(MiniKVError::ExtraArgument);
            }
            Ok(Comandos::Get {
                key: key.to_owned(),
            })
        }
        Some("length") => {
            if args.get(2).is_some() {
                return Err(MiniKVError::ExtraArgument);
            }
            Ok(Comandos::Length)
        }
        Some("snapshot") => {
            if args.get(2).is_some() {
                return Err(MiniKVError::ExtraArgument);
            }
            Ok(Comandos::Snapshot)
        }
        _ => Err(MiniKVError::UnknownCommand),
    }
}

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
            Ok(Comandos::Set {
                key,
                value: Some(v),
            }) => {
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
        assert!(matches!(resultado, Err(MiniKVError::MissingArgument)));
    }

    #[test]
    fn test_set_argumento_extra() {
        let resultado = parsear_comandos(args(&["minikv", "set", "clave1", "valor1", "extra"]));
        assert!(matches!(resultado, Err(MiniKVError::ExtraArgument)));
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
        assert!(matches!(resultado, Err(MiniKVError::MissingArgument)));
    }

    #[test]
    fn test_get_argumento_extra() {
        let resultado = parsear_comandos(args(&["minikv", "get", "clave1", "extra"]));
        assert!(matches!(resultado, Err(MiniKVError::ExtraArgument)));
    }

    #[test]
    fn test_length() {
        assert!(matches!(
            parsear_comandos(args(&["minikv", "length"])),
            Ok(Comandos::Length)
        ));
    }

    #[test]
    fn test_length_argumento_extra() {
        assert!(matches!(
            parsear_comandos(args(&["minikv", "length", "extra"])),
            Err(MiniKVError::ExtraArgument)
        ));
    }

    #[test]
    fn test_snapshot() {
        assert!(matches!(
            parsear_comandos(args(&["minikv", "snapshot"])),
            Ok(Comandos::Snapshot)
        ));
    }

    #[test]
    fn test_comando_invalido() {
        assert!(matches!(
            parsear_comandos(args(&["minikv", "delete"])),
            Err(MiniKVError::UnknownCommand)
        ));
    }
}
