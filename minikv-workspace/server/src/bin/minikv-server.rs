use server::handle_connection::handle_connection;
use minikv::almacenamiento::cargar_estado;
use minikv::errores::MiniKVError;
use std::collections::HashMap;
use std::env;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

fn run() -> Result<(), MiniKVError> {
    let args: Vec<String> = env::args().collect();

    let addr = args.get(1).ok_or(MiniKVError::InvalidArgs)?;

    let mut store: HashMap<String, Option<String>> = HashMap::new();
    cargar_estado(&mut store)?;

    let listener = TcpListener::bind(addr)
        .map_err(|_| MiniKVError::ServerSocketBinding)?;

    let store = Arc::new(Mutex::new(store));

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                let store = Arc::clone(&store);
                thread::spawn(move || handle_connection(s, store));
            }
            Err(e) => println!("{}", MiniKVError::ErrorIO(e.to_string())),
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}