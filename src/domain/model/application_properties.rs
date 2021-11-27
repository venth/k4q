use erased_serde::Deserializer;
use serde::Deserialize;

use crate::domain::model::K4fqError;

pub trait ApplicationProperties {
    fn properties_by(&self, prefix: &str) -> Result<Box<dyn ApplicationProperties>, K4fqError>;
    fn deserializer<'de>(&self) -> Box<dyn Deserializer<'de>>;
}


pub trait ApplicationPropertiesExt {
    fn try_collect<'de, T>(self) -> Result<T, K4fqError>
        where Self: Sized,
              T: Deserialize<'de>;
}

impl ApplicationPropertiesExt for &dyn ApplicationProperties {
    fn try_collect<'de, O>(self) -> Result<O, K4fqError>
        where Self: Sized,
              O: Deserialize<'de>
    {
        O::deserialize(self.deserializer())
            .map_err(|e| K4fqError::ConfigError(format!("{:?}", e)))
    }
}
