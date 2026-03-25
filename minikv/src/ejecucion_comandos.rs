use crate::almacenamiento::{escribir_log, escribir_snapshot};
use crate::comandos::Comandos;
use crate::errores::MiniKVError;
use std::collections::HashMap;

pub fn ejecutar_comando(
    comando: Comandos,
    store: &mut HashMap<String, Option<String>>,
) -> Result<(), MiniKVError> {
    match comando {
        Comandos::Set { key, value } => {
            escribir_log(&key, &value)?;
            store.insert(key, value);
            println!("OK");
        }
        Comandos::Get { key } => match store.get(&key) {
            Some(Some(val)) => println!("{}", val),
            _ => return Err(MiniKVError::NotFound),
        },
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
