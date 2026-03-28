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