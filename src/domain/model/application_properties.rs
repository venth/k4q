use std::error::Error;
use std::marker::PhantomData;
use std::time::Duration;

use serde::{Deserialize, Deserializer, Serialize};

use crate::domain::model::K4QError;

#[derive(Debug, Deserialize, Serialize)]
pub struct KafkaProperties {
    bootstrap: BootstrapProperties,
    message: MessageProperties,
}

pub trait ApplicationProperties {
    fn properties_by(&self, prefix: &str) -> Result<Box<dyn ApplicationProperties>, K4QError>;
}


pub trait CollectableProperties {
    fn try_collect<'de, T>(self) -> Result<T, K4QError> where T: Sized + Deserialize<'de>;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BootstrapProperties {
    servers: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageProperties {
    timeout: Duration,
}
