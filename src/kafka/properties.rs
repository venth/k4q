use std::time::Duration;

use serde;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct KafkaProperties {
    bootstrap: BootstrapProperties,
    message: MessageProperties,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct BootstrapProperties {
    servers: Vec<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct MessageProperties {
    timeout: Duration,
}
