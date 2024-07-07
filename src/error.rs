use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};
use tokio::io;

pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Error {
    pub message: String,
    pub kind: ErrorKind,
    pub status: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ErrorKind {
    NotFound,
    Serialization,
    Deserialization,
    Authentication,
    Other,
}

impl Error {
    pub fn not_found(message: impl Display) -> Self {
        Self {
            message: message.to_string(),
            kind: ErrorKind::NotFound,
            status: None,
        }
    }
    pub fn serialization(message: impl Display) -> Self {
        Self {
            message: message.to_string(),
            kind: ErrorKind::Serialization,
            status: None,
        }
    }
    pub fn deserialization(message: impl Display) -> Self {
        Self {
            message: message.to_string(),
            kind: ErrorKind::Deserialization,
            status: None,
        }
    }
    pub fn authentication(message: impl Display) -> Self {
        Self {
            message: message.to_string(),
            kind: ErrorKind::Authentication,
            status: None,
        }
    }
    pub fn other(message: impl Display) -> Self {
        Self {
            message: message.to_string(),
            kind: ErrorKind::Other,
            status: None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        let kind = match value.kind() {
            io::ErrorKind::NotFound => ErrorKind::NotFound,
            _ => ErrorKind::Other,
        };
        Self {
            message: format!("{value}"),
            status: None,
            kind,
        }
    }
}
impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self {
            message: format!("{value}"),
            kind: ErrorKind::Deserialization,
            status: None,
        }
    }
}
impl From<toml::de::Error> for Error {
    fn from(value: toml::de::Error) -> Self {
        Self {
            message: value.message().to_string(),
            kind: ErrorKind::Deserialization,
            status: None,
        }
    }
}
impl From<toml::ser::Error> for Error {
    fn from(value: toml::ser::Error) -> Self {
        Self {
            message: format!("{value}"),
            kind: ErrorKind::Serialization,
            status: None,
        }
    }
}
#[cfg(feature = "keyring")]
impl From<keyring::Error> for Error {
    fn from(value: keyring::Error) -> Self {
        Self {
            message: format!("{value}"),
            kind: ErrorKind::Authentication,
            status: None,
        }
    }
}
