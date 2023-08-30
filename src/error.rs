use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum SerializerError {
    Json(serde_json::Error),
    Yaml(serde_yaml::Error),
    Toml(toml::ser::Error),
}

impl Display for SerializerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                SerializerError::Json(err) => err.to_string(),
                SerializerError::Yaml(err) => err.to_string(),
                SerializerError::Toml(err) => err.to_string(),
            }
        )
    }
}

impl std::error::Error for SerializerError {}

impl From<serde_json::Error> for SerializerError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

impl From<serde_yaml::Error> for SerializerError {
    fn from(value: serde_yaml::Error) -> Self {
        Self::Yaml(value)
    }
}

impl From<toml::ser::Error> for SerializerError {
    fn from(value: toml::ser::Error) -> Self {
        Self::Toml(value)
    }
}
