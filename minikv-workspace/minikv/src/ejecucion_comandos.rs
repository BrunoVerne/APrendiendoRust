use crate::almacenamiento::{escribir_log, escribir_snapshot};
use crate::comandos::Comandos;
use crate::errores::MiniKVError;
use std::collections::HashMap;
use std::io::Write;

pub fn ejecutar_comando<W: Write>(
    comando: Comandos,
    store: &mut HashMap<String, Option<String>>,
    out: &mut W,
) -> Result<(), MiniKVError> {
    match comando {
        Comandos::Set { key, value } => {
            escribir_log(&key, &value)?;
            store.insert(key, value);
            writeln!(out, "OK").map_err(|e| MiniKVError::ErrorIO(e.to_string()))?;
        }
        Comandos::Get { key } => match store.get(&key) {
            Some(Some(val)) => {
                writeln!(out, "{}", val).map_err(|e| MiniKVError::ErrorIO(e.to_string()))?;
            }
            _ => return Err(MiniKVError::NotFound),
        },
        Comandos::Length => {
            let count = store.values().filter(|v| v.is_some()).count();
            writeln!(out, "{}", count).map_err(|e| MiniKVError::ErrorIO(e.to_string()))?;
        }
        Comandos::Snapshot => {
            escribir_snapshot(store)?;
            writeln!(out, "OK").map_err(|e| MiniKVError::ErrorIO(e.to_string()))?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn store_vacio() -> HashMap<String, Option<String>> {
        HashMap::new()
    }

    fn output_a_string(out: Vec<u8>) -> String {
        String::from_utf8(out).unwrap().trim().to_string()
    }

    #[test]
    fn test_set_get() {
        let mut store = store_vacio();
        let mut out = Vec::new();

        ejecutar_comando(
            Comandos::Set {
                key: "clave".to_string(),
                value: Some("valor".to_string()),
            },
            &mut store,
            &mut out,
        )
        .unwrap();
        assert_eq!(output_a_string(out), "OK");

        let mut out = Vec::new();
        ejecutar_comando(
            Comandos::Get {
                key: "clave".to_string(),
            },
            &mut store,
            &mut out,
        )
        .unwrap();
        assert_eq!(output_a_string(out), "valor");
    }

    #[test]
    fn test_unset() {
        let mut store = store_vacio();
        let mut out = Vec::new();

        // set sin valor debe retornar OK
        ejecutar_comando(
            Comandos::Set {
                key: "clave".to_string(),
                value: None,
            },
            &mut store,
            &mut out,
        )
        .unwrap();
        assert_eq!(output_a_string(out), "OK");

        // get de clave sin valor debe retornar NOT FOUND
        let mut out = Vec::new();
        let err = ejecutar_comando(
            Comandos::Get {
                key: "clave".to_string(),
            },
            &mut store,
            &mut out,
        )
        .unwrap_err();
        assert!(matches!(err, MiniKVError::NotFound));
    }

    #[test]
    fn test_get_not_found() {
        let mut store = store_vacio();
        let mut out = Vec::new();

        let err = ejecutar_comando(
            Comandos::Get {
                key: "inexistente".to_string(),
            },
            &mut store,
            &mut out,
        )
        .unwrap_err();
        assert!(matches!(err, MiniKVError::NotFound));
    }

    #[test]
    fn test_length_vacio() {
        let mut store = store_vacio();
        let mut out = Vec::new();

        ejecutar_comando(Comandos::Length, &mut store, &mut out).unwrap();
        assert_eq!(output_a_string(out), "0");
    }

    #[test]
    fn test_length_con_valores() {
        let mut store = store_vacio();
        let mut out = Vec::new();

        ejecutar_comando(
            Comandos::Set {
                key: "a".to_string(),
                value: Some("1".to_string()),
            },
            &mut store,
            &mut out,
        )
        .unwrap();
        ejecutar_comando(
            Comandos::Set {
                key: "b".to_string(),
                value: Some("2".to_string()),
            },
            &mut store,
            &mut out,
        )
        .unwrap();

        let mut out = Vec::new();
        ejecutar_comando(Comandos::Length, &mut store, &mut out).unwrap();
        assert_eq!(output_a_string(out), "2");
    }

    #[test]
    fn test_length_descuenta_unset() {
        let mut store = store_vacio();
        let mut out = Vec::new();

        ejecutar_comando(
            Comandos::Set {
                key: "a".to_string(),
                value: Some("1".to_string()),
            },
            &mut store,
            &mut out,
        )
        .unwrap();
        ejecutar_comando(
            Comandos::Set {
                key: "a".to_string(),
                value: None,
            },
            &mut store,
            &mut out,
        )
        .unwrap();

        let mut out = Vec::new();
        ejecutar_comando(Comandos::Length, &mut store, &mut out).unwrap();
        assert_eq!(output_a_string(out), "0");
    }

    #[test]
    fn test_set_sobreescribe() {
        let mut store = store_vacio();
        let mut out = Vec::new();

        ejecutar_comando(
            Comandos::Set {
                key: "clave".to_string(),
                value: Some("viejo".to_string()),
            },
            &mut store,
            &mut out,
        )
        .unwrap();
        ejecutar_comando(
            Comandos::Set {
                key: "clave".to_string(),
                value: Some("nuevo".to_string()),
            },
            &mut store,
            &mut out,
        )
        .unwrap();

        let mut out = Vec::new();
        ejecutar_comando(
            Comandos::Get {
                key: "clave".to_string(),
            },
            &mut store,
            &mut out,
        )
        .unwrap();
        assert_eq!(output_a_string(out), "nuevo");
    }
}
