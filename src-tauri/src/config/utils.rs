use std::env;

use crate::errors::ConfigError;

pub fn get_var(key: &str) -> Result<String, ConfigError> {
    env::var(key).map_err(|_| ConfigError::EnvVar(key.into()))
}

// Convierte a bool (acepta "true"/"false", "1"/"0")
pub fn get_bool(key: &str) -> Result<bool, ConfigError> {
    let v = get_var(key)?;
    match v.to_ascii_lowercase().as_str() {
        "true" | "1" => Ok(true),
        "false" | "0" => Ok(false),
        _ => Err(ConfigError::EnvVar(format!("{key}=({v}) no es booleano"))),
    }
}
