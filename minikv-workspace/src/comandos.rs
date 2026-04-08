/// Representa los comandos disponibles en MiniKV.
pub enum Comandos {
    /// Almacena una clave con un valor opcional en el store.
    /// Si el valor es `None`, la clave queda registrada sin valor.
    Set { key: String, value: Option<String> },

    /// Obtiene el valor asociado a una clave del store.
    Get { key: String },

    /// Retorna la cantidad de claves con valor presente en el store.
    Length,

    /// Persiste el estado actual del store en disco y limpia el log.
    Snapshot,
}
