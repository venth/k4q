use std::error::Error;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum K4QError {
    ConfigError(String),
    KafkaError(String),
}

impl Display for K4QError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            K4QError::ConfigError(message) => write!(f, "K4QError::ConfigError{{ message: {} }}", message),
            K4QError::KafkaError(message) => write!(f, "K4QError::KafkaError{{ message: {} }}", message),
        }

    }
}

impl Error for K4QError {

}
