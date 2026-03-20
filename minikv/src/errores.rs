pub enum MiniKVError {
    ComandoInvalido,
    FaltaKey,
    ErrorIO(String),
}

impl std::fmt::Display for MiniKVError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MiniKVError::ComandoInvalido  => write!(f, "Comando inválido"),
            MiniKVError::FaltaKey         => write!(f, "Falta key"),
            MiniKVError::ErrorIO(e)       => write!(f, "Error de IO: {}", e),
        }
    }
}