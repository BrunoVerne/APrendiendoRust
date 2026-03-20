use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use crate::errores::MiniKVError;

fn formatear(s: &str) -> String {
    format!("\"{}\"", s.replace('"', "\\\""))
}

fn desformatear(s: &str) -> String {
    let s = s.strip_prefix('"').unwrap_or(s);
    let s = s.strip_suffix('"').unwrap_or(s);
    s.replace("\\\"", "\"")
}

fn parsear_linea(line: &str) -> Option<(String, Option<String>)> {
    if line.is_empty() {
        return None;
    }
    let line = if line.starts_with("set ") { &line[4..] } else { line };
    let mut partes = line.splitn(2, "\" \"");
    match (partes.next(), partes.next()) {
        (Some(k), Some(v)) => Some((desformatear(k), Some(desformatear(v)))),
        (Some(k), None)    => Some((desformatear(k), None)),
        _                  => None,
    }
}

pub fn cargar_estado(store: &mut HashMap<String, Option<String>>) {
    for archivo in &[".minikv.data", ".minikv.log"] {
        if let Ok(file) = File::open(archivo) {
            for line in BufReader::new(file).lines() {
                if let Ok(linea) = line {
                    if let Some((k, v)) = parsear_linea(&linea) {
                        store.insert(k, v);
                    }
                }
            }
        }
    }
}

pub fn escribir_log(key: &str, value: &Option<String>) -> Result<(), MiniKVError> {
    let mut log = OpenOptions::new()
        .create(true)
        .append(true)
        .open(".minikv.log")
        .map_err(|e| MiniKVError::ErrorIO(e.to_string()))?;

    match value {
        Some(v) => writeln!(log, "set {} {}", formatear(key), formatear(v)),
        None    => writeln!(log, "set {}", formatear(key)),
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

    #[test]
    fn test_parsear_linea_clave_valor() {
        assert_eq!(
            parsear_linea("\"clave1\" \"valor1\""),
            Some(("clave1".to_owned(), Some("valor1".to_owned())))
        );
    }

    #[test]
    fn test_parsear_linea_solo_clave() {
        assert_eq!(
            parsear_linea("\"clave1\""),
            Some(("clave1".to_owned(), None))
        );
    }

    #[test]
    fn test_parsear_linea_con_prefijo_set() {
        assert_eq!(
            parsear_linea("set \"clave1\" \"valor1\""),
            Some(("clave1".to_owned(), Some("valor1".to_owned())))
        );
    }

    #[test]
    fn test_parsear_linea_con_espacios() {
        assert_eq!(
            parsear_linea("\"clave 1\" \"valor 1\""),
            Some(("clave 1".to_owned(), Some("valor 1".to_owned())))
        );
    }

   

    #[test]
    fn test_parsear_linea_vacia() {
        assert_eq!(parsear_linea(""), None);
    }
}