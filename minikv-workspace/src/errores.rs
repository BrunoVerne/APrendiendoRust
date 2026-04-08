//! Representa los posibles errores que puede producir MiniKV.
#[derive(Debug)]
pub enum MiniKVError {
    /// La clave solicitada no existe o no tiene valor en el store.
    NotFound,

    /// Se proporcionó un argumento de más en el comando.
    ExtraArgument,

    /// El archivo `.minikv.data` tiene un formato inválido.
    InvalidDataFile,

    /// El archivo `.minikv.log` tiene un formato inválido.
    InvalidLogFile,

    /// Falta un argumento requerido en el comando.
    MissingArgument,

    /// El comando ingresado no es reconocido por MiniKV.
    UnknownCommand,

    /// Error de entrada/salida con su descripción.
    ErrorIO(String),

    //Error de direccion dada por cliente
    ClientSocketBinding,

    //Error de direccion dada por servidor
    ServerSocketBinding,

    InvalidArgs,
    /// El servidor tarda demasiado en contestar.
    Timeout,
    /// La conexión se cierra de forma repentina.
    ConnectionClosed,
}
impl std::fmt::Display for MiniKVError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            // Errores de cliente
            MiniKVError::NotFound => write!(f, "ERROR \"NOT FOUND\""),
            MiniKVError::ExtraArgument => write!(f, "ERROR \"EXTRA ARGUMENT\""),
            MiniKVError::MissingArgument => write!(f, "ERROR \"MISSING ARGUMENT\""),
            MiniKVError::UnknownCommand => write!(f, "ERROR \"UNKNOWN COMMAND\""),
            // Errores del server
            MiniKVError::InvalidDataFile => write!(f, "ERROR \"INVALID DATA FILE\""),
            MiniKVError::InvalidLogFile => write!(f, "ERROR \"INVALID LOG FILE\""),
            MiniKVError::ServerSocketBinding => write!(f, "ERROR \"SERVER SOCKET BINDING\""),
            MiniKVError::InvalidArgs => write!(f, "ERROR \"INVALID ARGS\""),
            // Errores de comunicacion
            MiniKVError::ClientSocketBinding => write!(f, "ERROR \"CLIENT SOCKET BINDING\""),
            MiniKVError::Timeout => write!(f, "ERROR \"TIMEOUT\""),
            MiniKVError::ConnectionClosed => write!(f, "ERROR \"CONNECTION CLOSED\""),
            MiniKVError::ErrorIO(e) => write!(f, "ERROR \"{}\"", e),
        }
    }
}
