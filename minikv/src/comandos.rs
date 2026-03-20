pub enum Comandos {
    Set { key: String, value: Option<String> },
    Get { key: String },
    Length,
    Snapshot,
}