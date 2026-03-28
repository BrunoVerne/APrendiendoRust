mod almacenamiento;
mod comandos;
mod ejecucion_comandos;
mod errores;
mod parseo_comandos;

use almacenamiento::cargar_estado;
use ejecucion_comandos::ejecutar_comando;
use parseo_comandos::parsear_comandos;
use std::collections::HashMap;
use std::env;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut store: HashMap<String, Option<String>> = HashMap::new();
    let mut out = io::stdout();

    if let Err(e) = cargar_estado(&mut store) {
        println!("{}", e);
        return;
    }

    match parsear_comandos(args) {
        Ok(cmd) => {
            if let Err(e) = ejecutar_comando(cmd, &mut store, &mut out) {
                println!("{}", e);
            }
        }
        Err(e) => println!("{}", e),
    }
}