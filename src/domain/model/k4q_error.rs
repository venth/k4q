use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum K4QError {
    ConfigError(String),
}