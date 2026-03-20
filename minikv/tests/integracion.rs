use std::collections::HashMap;

// Simula el store en memoria sin tocar archivos
fn ejecutar(store: &mut HashMap<String, Option<String>>, args: &[&str]) -> String {
    match args.get(0) {
        Some(&"set") => {
            let key = args.get(1).unwrap().to_string();
            let value = args.get(2).map(|v| v.to_string());
            store.insert(key, value);
            "OK".to_string()
        }
        Some(&"get") => {
            let key = args.get(1).unwrap();
            match store.get(*key) {
                Some(Some(val)) => val.to_string(),
                _               => "NOT FOUND".to_string(),
            }
        }
        Some(&"length") => {
            store.values().filter(|v| v.is_some()).count().to_string()
        }
        _ => "Comando inválido".to_string(),
    }
}

#[test]
fn test_set_y_get() {
    let mut store = HashMap::new();
    assert_eq!(ejecutar(&mut store, &["set", "clave1", "valor1"]), "OK");
    assert_eq!(ejecutar(&mut store, &["get", "clave1"]), "valor1");
}

#[test]
fn test_get_inexistente() {
    let mut store = HashMap::new();
    assert_eq!(ejecutar(&mut store, &["get", "inexistente"]), "NOT FOUND");
}

#[test]
fn test_set_sobreescribe_valor() {
    let mut store = HashMap::new();
    ejecutar(&mut store, &["set", "clave1", "valor1"]);
    ejecutar(&mut store, &["set", "clave1", "valor2"]);
    assert_eq!(ejecutar(&mut store, &["get", "clave1"]), "valor2");
}

#[test]
fn test_set_sin_valor_desasocia() {
    let mut store = HashMap::new();
    ejecutar(&mut store, &["set", "clave1", "valor1"]);
    ejecutar(&mut store, &["set", "clave1"]);
    assert_eq!(ejecutar(&mut store, &["get", "clave1"]), "NOT FOUND");
}

#[test]
fn test_length_con_valores() {
    let mut store = HashMap::new();
    ejecutar(&mut store, &["set", "clave1", "valor1"]);
    ejecutar(&mut store, &["set", "clave2", "valor2"]);
    assert_eq!(ejecutar(&mut store, &["length"]), "2");
}

#[test]
fn test_length_excluye_desasociadas() {
    let mut store = HashMap::new();
    ejecutar(&mut store, &["set", "clave1", "valor1"]);
    ejecutar(&mut store, &["set", "clave2", "valor2"]);
    ejecutar(&mut store, &["set", "clave2"]);
    assert_eq!(ejecutar(&mut store, &["length"]), "1");
}

#[test]
fn test_length_vacio() {
    let mut store = HashMap::new();
    assert_eq!(ejecutar(&mut store, &["length"]), "0");
}

#[test]
fn test_set_clave_con_espacios() {
    let mut store = HashMap::new();
    ejecutar(&mut store, &["set", "clave 1", "valor 1"]);
    assert_eq!(ejecutar(&mut store, &["get", "clave 1"]), "valor 1");
}

#[test]
fn test_set_clave_con_comillas() {
    let mut store = HashMap::new();
    ejecutar(&mut store, &["set", "clave \"A\"", "valor \"A\""]);
    assert_eq!(ejecutar(&mut store, &["get", "clave \"A\""]), "valor \"A\"");
}

#[test]
fn test_flujo_completo() {
    let mut store = HashMap::new();
    ejecutar(&mut store, &["set", "clave1", "valor1"]);
    ejecutar(&mut store, &["set", "clave2", "valor2"]);
    assert_eq!(ejecutar(&mut store, &["length"]), "2");
    ejecutar(&mut store, &["set", "clave1"]);
    assert_eq!(ejecutar(&mut store, &["length"]), "1");
    assert_eq!(ejecutar(&mut store, &["get", "clave1"]), "NOT FOUND");
    assert_eq!(ejecutar(&mut store, &["get", "clave2"]), "valor2");
}