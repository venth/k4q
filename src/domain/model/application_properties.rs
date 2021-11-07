use serde::Deserialize;

use crate::domain::model::K4QError;

pub trait ApplicationProperties {
    fn properties_by(&self, prefix: &str) -> Result<Box<dyn ApplicationProperties>, K4QError>;
}


pub trait CollectableProperties {
    fn try_collect<'de, T>(self) -> Result<T, K4QError> where T: Sized + Deserialize<'de>;
}


impl<T> CollectableProperties for ApplicationConfig<T> where
    T: CollectableProperties + ApplicationProperties {
    fn try_collect<'de, V>(self) -> Result<V, K4QError> where V: Sized + Deserialize<'de> {
        self.config
            .try_collect()
    }
}

impl CollectableProperties for &dyn ApplicationProperties {
    fn try_collect<'de, T>(self) -> Result<T, K4QError>
        where
            Self: CollectableProperties, T: Sized + Deserialize<'de>
    {
        self.try_collect()
    }
}

impl<T: ApplicationProperties + CollectableProperties> ApplicationProperties for ApplicationConfig<T> {
    fn properties_by(&self, prefix: &str) -> Result<Box<dyn ApplicationProperties>, K4QError> {
        self.config.properties_by(prefix)
    }
}


pub struct ApplicationConfig<T>
    where
        T: CollectableProperties + ApplicationProperties,
{
    config: T,
}

impl<T> ApplicationConfig<T>
    where
        T: CollectableProperties + ApplicationProperties,
{
    pub fn new(config: T) -> Self {
        ApplicationConfig { config }
    }
}
