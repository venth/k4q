use std::error::Error;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum K4fqError {
    ConfigError(String),
    KafkaError(String),
}

impl Display for K4fqError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            K4fqError::ConfigError(message) => write!(f, "K4fqError::ConfigError{{ message: {} }}", message),
            K4fqError::KafkaError(message) => write!(f, "K4fqError::KafkaError{{ message: {} }}", message),
        }

    }
}

impl Error for K4fqError {

}
