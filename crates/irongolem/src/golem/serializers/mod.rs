use std::fs;
use std::path::Path;

use serde_json::{Map, Value};

pub const CLASS_PATH_KEY: &str = "_class_path";
pub const MODULE_X_NAME_DELIMITER: &str = "/";

pub fn dump_path_to_obj(module: &str, name: &str) -> serde_json::Map<String, serde_json::Value> {
    let mut map = Map::new();
    map.insert(
        CLASS_PATH_KEY.into(),
        Value::String(format!("{module}{MODULE_X_NAME_DELIMITER}{name}")),
    );
    map
}

pub fn default_save<T: serde::Serialize>(obj: &T, json_file_path: Option<&Path>) -> String {
    let value = serde_json::to_value(obj).unwrap_or(Value::Null);
    let json = serde_json::to_string_pretty(&value).unwrap_or_else(|_| "{}".into());
    if let Some(path) = json_file_path {
        let _ = fs::write(path, &json);
        json
    } else {
        json
    }
}

pub fn default_load<T: serde::de::DeserializeOwned>(
    path_or_str: &str,
) -> Result<T, serde_json::Error> {
    let content = if Path::new(path_or_str).exists() {
        fs::read_to_string(path_or_str).unwrap_or_else(|_| path_or_str.to_string())
    } else {
        path_or_str.to_string()
    };
    serde_json::from_str(&content)
}

pub fn encode_value<T: serde::Serialize>(value: &T) -> Map<String, Value> {
    match serde_json::to_value(value) {
        Ok(Value::Object(map)) => map,
        Ok(_) => Map::new(),
        Err(_) => Map::new(),
    }
}

pub fn register_serializable<T>() -> fn(&T) -> Map<String, Value>
where
    T: serde::Serialize,
{
    encode_value
}
