use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Debug, thiserror::Error)]
pub enum PacksightError {
    #[error("CRG Key Error: {0} is not a valid key.")]
    CRGKeyError(String),
    #[error("{0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("{0}")]
    SerdePathToError(#[from] serde_path_to_error::Error<serde_json::Error>),
    #[error("An error occurred: {0}")]
    Error(String),
}

pub type Result<T, E = PacksightError> = anyhow::Result<T, E>;

impl Serialize for PacksightError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&match self {
            PacksightError::CRGKeyError(str) => str.to_string(),
            PacksightError::SerdeError(serde_err) => serde_err.to_string(),
            PacksightError::Error(str) => str.to_string(),
            PacksightError::SerdePathToError(serde_path_to_err) => serde_path_to_err.to_string(),
        })
    }
}

impl<'de> Deserialize<'de> for PacksightError {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(PacksightErrorVisitor)
    }
}

struct PacksightErrorVisitor;

impl<'de> Visitor<'de> for PacksightErrorVisitor {
    type Value = PacksightError;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(PacksightError::Error(v.into()))
    }

    fn visit_string<E>(self, v: String) -> std::result::Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(PacksightError::Error(v))
    }
}
