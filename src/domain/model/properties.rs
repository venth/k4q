use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct KafkaProperties {
    bootstrap: BootstrapProperties,
    message: MessageProperties,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Properties {
    kafka: KafkaProperties,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BootstrapProperties {
    servers: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageProperties {
    timeout: Duration,
}
