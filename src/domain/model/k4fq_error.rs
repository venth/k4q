use std::error::Error;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum K4fqError {
    ConfigError(String),
    KafkaError(String),
    NotSupported,
}

unsafe impl Send for K4fqError {}

impl Display for K4fqError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            K4fqError::ConfigError(message) => write!(f, "K4fqError::ConfigError{{ message: {} }}", message),
            K4fqError::KafkaError(message) => write!(f, "K4fqError::KafkaError{{ message: {} }}", message),
            _ => write!(f, "K4fqError - operation not supported yet")
        }

    }
}

impl Error for K4fqError {

}
