use std::fs;
use std::path::Path;

use serde_json::{Map, Value};

pub const CLASS_PATH_KEY: &str = "_class_path";
pub const MODULE_X_NAME_DELIMITER: &str = "/";

#[derive(Debug)]
pub enum SerializerError {
    Json(serde_json::Error),
    Io(std::io::Error),
    ExpectedObject,
}

impl std::fmt::Display for SerializerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(err) => write!(f, "{err}"),
            Self::Io(err) => write!(f, "{err}"),
            Self::ExpectedObject => write!(f, "expected a JSON object"),
        }
    }
}

impl std::error::Error for SerializerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Json(err) => Some(err),
            Self::Io(err) => Some(err),
            Self::ExpectedObject => None,
        }
    }
}

impl From<serde_json::Error> for SerializerError {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}

impl From<std::io::Error> for SerializerError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

pub fn dump_path_to_obj(module: &str, name: &str) -> serde_json::Map<String, serde_json::Value> {
    let mut map = Map::new();
    map.insert(
        CLASS_PATH_KEY.into(),
        Value::String(format!("{module}{MODULE_X_NAME_DELIMITER}{name}")),
    );
    map
}

pub fn default_save<T: serde::Serialize>(
    obj: &T,
    json_file_path: Option<&Path>,
) -> Result<String, SerializerError> {
    let json = serde_json::to_string_pretty(obj)?;
    if let Some(path) = json_file_path {
        fs::write(path, &json)?;
    }
    Ok(json)
}

pub fn default_load<T: serde::de::DeserializeOwned>(
    path_or_str: &str,
) -> Result<T, SerializerError> {
    let content = if Path::new(path_or_str).exists() {
        fs::read_to_string(path_or_str)?
    } else {
        path_or_str.to_string()
    };
    Ok(serde_json::from_str(&content)?)
}

pub fn encode_value<T: serde::Serialize>(value: &T) -> Result<Map<String, Value>, SerializerError> {
    match serde_json::to_value(value)? {
        Value::Object(map) => Ok(map),
        _ => Err(SerializerError::ExpectedObject),
    }
}

pub fn register_serializable<T>() -> fn(&T) -> Result<Map<String, Value>, SerializerError>
where
    T: serde::Serialize,
{
    encode_value
}
