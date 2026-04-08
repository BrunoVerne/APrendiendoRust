use minikv::errores::MiniKVError;
use std::env;
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::time::Duration;

/// Timeout en segundos para esperar respuesta del servidor.
const TIMEOUT_SECS: u64 = 5;

fn run() -> Result<(), MiniKVError> {
    let args: Vec<String> = env::args().collect();
    let addr = args.get(1).ok_or(MiniKVError::InvalidArgs)?;

    let stream = TcpStream::connect(addr).map_err(|_| MiniKVError::ClientSocketBinding)?;

    stream
        .set_read_timeout(Some(Duration::from_secs(TIMEOUT_SECS)))
        .map_err(|e| MiniKVError::ErrorIO(e.to_string()))?;

    let mut writer = stream
        .try_clone()
        .map_err(|e| MiniKVError::ErrorIO(e.to_string()))?;
    let mut reader = BufReader::new(stream);

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line.map_err(|e| MiniKVError::ErrorIO(e.to_string()))?;

        writeln!(writer, "{}", line).map_err(|_| MiniKVError::ConnectionClosed)?;

        let mut respuesta = String::new();
        match reader.read_line(&mut respuesta) {
            Ok(0) => {
                println!("{}", MiniKVError::ConnectionClosed);
                return Ok(());
            }
            Ok(_) => print!("{}", respuesta),
            Err(e)
                if e.kind() == io::ErrorKind::WouldBlock || e.kind() == io::ErrorKind::TimedOut =>
            {
                println!("{}", MiniKVError::Timeout);
                return Ok(());
            }
            Err(_) => {
                println!("{}", MiniKVError::ConnectionClosed);
                return Ok(());
            }
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}
