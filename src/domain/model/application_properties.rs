use erased_serde::Deserializer;
use serde::Deserialize;

use crate::domain::model::K4QError;

pub trait ApplicationProperties {
    fn properties_by(&self, prefix: &str) -> Result<Box<dyn ApplicationProperties>, K4QError>;
    fn deserializer<'de>(&self) -> Box<dyn Deserializer<'de>>;
}


pub trait ApplicationPropertiesExt {
    fn try_collect<'de, T>(self) -> Result<T, K4QError>
        where Self: Sized,
              T: Deserialize<'de>;
}

impl ApplicationPropertiesExt for &dyn ApplicationProperties {
    fn try_collect<'de, O>(self) -> Result<O, K4QError>
        where Self: Sized,
              O: Deserialize<'de>
    {
        O::deserialize(self.deserializer())
            .map_err(|e| K4QError::ConfigError(format!("{:?}", e)))
    }
}
