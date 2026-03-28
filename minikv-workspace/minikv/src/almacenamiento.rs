use crate::errores::MiniKVError;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

fn formatear(s: &str) -> String {
    format!("\"{}\"", s.replace('"', "\\\""))
}

fn desformatear(s: &str) -> String {
    let s = s.strip_prefix('"').unwrap_or(s);
    let s = s.strip_suffix('"').unwrap_or(s);
    s.replace("\\\"", "\"")
}

fn parsear_linea_data(line: &str) -> Option<(String, Option<String>)> {
    if line.is_empty() {
        return None;
    }
    let mut partes = line.splitn(2, "\" \"");
    match (partes.next(), partes.next()) {
        (Some(k), Some(v)) => Some((desformatear(k), Some(desformatear(v)))),
        _ => None, // data solo acepta pares clave-valor completos
    }
}

fn parsear_linea_log(line: &str) -> Option<(String, Option<String>)> {
    let line = line.strip_prefix("set ")?;
    let mut partes = line.splitn(2, "\" \"");
    match (partes.next(), partes.next()) {
        (Some(k), Some(v)) => Some((desformatear(k), Some(desformatear(v)))),
        (Some(k), None) => Some((desformatear(k), None)),
        _ => None,
    }
}
pub fn cargar_estado(store: &mut HashMap<String, Option<String>>) -> Result<(), MiniKVError> {
    cargar_archivo(
        ".minikv.data",
        parsear_linea_data,
        || MiniKVError::InvalidDataFile,
        store,
    )?;
    cargar_archivo(
        ".minikv.log",
        parsear_linea_log,
        || MiniKVError::InvalidLogFile,
        store,
    )?;
    Ok(())
}

fn cargar_archivo<P, E>(
    path: &str,
    parsear: P,
    error: E,
    store: &mut HashMap<String, Option<String>>,
) -> Result<(), MiniKVError>
where
    P: Fn(&str) -> Option<(String, Option<String>)>,
    E: Fn() -> MiniKVError,
{
    if let Ok(file) = File::open(path) {
        for line in BufReader::new(file).lines() {
            let linea = line.map_err(|_| error())?;
            if linea.is_empty() {
                continue;
            }
            match parsear(&linea) {
                Some((k, v)) => {
                    store.insert(k, v);
                }
                None => return Err(error()),
            }
        }
    }
    Ok(())
}

pub fn escribir_log(key: &str, value: &Option<String>) -> Result<(), MiniKVError> {
    let mut log = OpenOptions::new()
        .create(true)
        .append(true)
        .open(".minikv.log")
        .map_err(|e| MiniKVError::ErrorIO(e.to_string()))?;

    match value {
        Some(v) => writeln!(log, "set {} {}", formatear(key), formatear(v)),
        None => writeln!(log, "set {}", formatear(key)),
    }
    .map_err(|e| MiniKVError::ErrorIO(e.to_string()))
}

pub fn escribir_snapshot(store: &HashMap<String, Option<String>>) -> Result<(), MiniKVError> {
    let mut data = File::create(".minikv.data").map_err(|e| MiniKVError::ErrorIO(e.to_string()))?;
    for (k, v) in store.iter() {
        if let Some(val) = v {
            writeln!(data, "{} {}", formatear(k), formatear(val))
                .map_err(|e| MiniKVError::ErrorIO(e.to_string()))?;
        }
    }
    File::create(".minikv.log").map_err(|e| MiniKVError::ErrorIO(e.to_string()))?;
    Ok(())
}

//-----------------------------------------------------------TEST-----------------------------------------------------------//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formatear_simple() {
        assert_eq!(formatear("clave1"), "\"clave1\"");
    }

    #[test]
    fn test_formatear_con_espacios() {
        assert_eq!(formatear("clave 1"), "\"clave 1\"");
    }

    #[test]
    fn test_formatear_con_comillas() {
        assert_eq!(formatear("clave \"A\""), "\"clave \\\"A\\\"\"");
    }

    #[test]
    fn test_desformatear_simple() {
        assert_eq!(desformatear("\"clave1\""), "clave1");
    }

    #[test]
    fn test_desformatear_con_espacios() {
        assert_eq!(desformatear("\"clave 1\""), "clave 1");
    }

    #[test]
    fn test_desformatear_con_comillas() {
        assert_eq!(desformatear("\"clave \\\"A\\\"\""), "clave \"A\"");
    }

    // --- data: solo acepta pares clave-valor completos ---

    #[test]
    fn test_data_solo_clave_es_invalido() {
        assert_eq!(parsear_linea_data("\"clave1\""), None);
    }

    #[test]
    fn test_data_con_espacios() {
        assert_eq!(
            parsear_linea_data("\"clave 1\" \"valor 1\""),
            Some(("clave 1".to_owned(), Some("valor 1".to_owned())))
        );
    }

    #[test]
    fn test_data_vacia_es_invalido() {
        assert_eq!(parsear_linea_data(""), None);
    }

    // --- log: solo acepta lineas con prefijo "set" ---

    #[test]
    fn test_log_set_clave_valor() {
        assert_eq!(
            parsear_linea_log("set \"clave1\" \"valor1\""),
            Some(("clave1".to_owned(), Some("valor1".to_owned())))
        );
    }

    #[test]
    fn test_log_set_solo_clave() {
        assert_eq!(
            parsear_linea_log("set \"clave1\""),
            Some(("clave1".to_owned(), None))
        );
    }

    #[test]
    fn test_log_sin_prefijo_set_es_invalido() {
        assert_eq!(parsear_linea_log("get \"clave1\""), None);
    }

    #[test]
    fn test_log_con_espacios() {
        assert_eq!(
            parsear_linea_log("set \"clave 1\" \"valor 1\""),
            Some(("clave 1".to_owned(), Some("valor 1".to_owned())))
        );
    }

    #[test]
    fn test_log_vacia_es_invalido() {
        assert_eq!(parsear_linea_log(""), None);
    }

    #[test]
    fn test_data_clave_valor() {
        assert_eq!(
            parsear_linea_data("\"clave1\" \"valor1\""),
            Some(("clave1".to_owned(), Some("valor1".to_owned())))
        );
    }

    #[test]
    fn test_data_solo_clave_invalido() {
        // replica error-invalid-data: "\"k\"" en .minikv.data
        assert_eq!(parsear_linea_data("\"k\""), None);
    }

    #[test]
    fn test_data_formato_incorrecto() {
        // replica error-invalid-data-2: "set a b," en .minikv.data
        assert_eq!(parsear_linea_data("set a b,"), None);
    }

    #[test]
    fn test_log_operacion_invalida() {
        // replica error-invalid-log: "get \"k\"" en .minikv.log
        assert_eq!(parsear_linea_log("get \"k\""), None);
    }
}
