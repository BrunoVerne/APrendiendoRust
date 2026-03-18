use std::env;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

enum Comandos {
    Set { key: String, value: Option<String> },
    Get { key: String },
    Length,
    Snapshot,
}

fn parsear_comandos(args: Vec<String>) -> Result<Comandos, String> {

    let comando: Option<&str> = args.get(1).map(|s| s.as_str()); //comando puede ser o Some(String) o None
    match comando
    {

        Some("set") => {
            let key = args.get(2).ok_or("Falta key")?;

            let option_valor: Option<&String> =  args.get(3);

            match option_valor {
                
                Some(value) => Ok(Comandos::Set { key: key.to_owned(), value: Some(value.to_owned()) }),

                None => Ok(Comandos::Set { key: key.to_owned(), value: None }),
            }
        }


        Some("get") => {
            let key = args.get(2).ok_or("Falta key")?;
            Ok(Comandos::Get { key: key.to_owned() })
        }


        Some("length")   => Ok(Comandos::Length),



        Some("snapshot") => Ok(Comandos::Snapshot),


        _ => Err("Comando inválido".to_string()),
    }
}



fn formatear(s: &str) -> String {
    format!("\"{}\"", s.replace('"', "\\\""))
}

fn desformatear(s: &str) -> String {
    s.trim_matches('"').replace("\\\"", "\"")
}



fn parsear_linea(line: &str) -> Option<(String, Option<String>)> {
    let line = if line.starts_with("set ") { &line[4..] } else { line };
    let mut partes = line.splitn(2, "\" \"");
    match (partes.next(), partes.next()) {
        (Some(k), Some(v)) => Some((desformatear(k), Some(desformatear(v)))),
        (Some(k), None)    => Some((desformatear(k), None)),
        _                  => None,
    }
}

fn cargar_estado(store: &mut HashMap<String, Option<String>>) {
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



fn ejecutar_comando(comando: Comandos, store: &mut HashMap<String, Option<String>>) -> Result<(), String> {


    match comando {




        Comandos::Set { key, value } => {
            let mut log = OpenOptions::new()
                .create(true)
                .append(true)
                .open(".minikv.log")
                .map_err(|e| e.to_string())?;

            match &value {
                Some(v) => writeln!(log, "set {} {}", formatear(&key), formatear(v)).map_err(|e| e.to_string())?,
                None    => writeln!(log, "set {}", formatear(&key)).map_err(|e| e.to_string())?,
            }


            store.insert(key, value);
            println!("OK");
        }



        Comandos::Get { key } => {
            match store.get(&key) {

                Some(Some(val)) => println!("{}", val),
                _  => println!("NOT FOUND"),
            }

        }



        Comandos::Length => {
            println!("{}", store.values().filter(|v| v.is_some()).count());
        }


        Comandos::Snapshot => {

            let mut data = File::create(".minikv.data").map_err(|e| e.to_string())?;
            for (k, v) in store.iter() {
                if let Some(val) = v {
                    writeln!(data, "{} {}", formatear(k), formatear(val)).map_err(|e| e.to_string())?;
                }
}
            File::create(".minikv.log").map_err(|e| e.to_string())?;



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