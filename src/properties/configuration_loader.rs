use std::collections::HashMap;
use std::path::Path;

use config::{Config, ConfigError, Source, Value};
use serde::Deserialize;

use crate::domain::model::{ApplicationConfig, ApplicationProperties, CollectableProperties, K4QError};
use crate::domain::ports;

#[derive(shaku::Component)]
#[shaku(interface = ports::PropertiesLoader)]
pub struct ConfigurationLoader {}

impl ports::PropertiesLoader for ConfigurationLoader {
    fn load(&self, config_location: &Path) -> Result<Box<dyn ApplicationProperties>, K4QError> {
        Config::default()
            .with_merged(config::File::with_name(config_location.to_str().unwrap()))
            .map(|c| PartialConfig { config: c })
            .map(ApplicationConfig::<PartialConfig>::new)
            .map(Box::new)
            .map(|c| c as Box<dyn ApplicationProperties>)
            .map_err(ConfigurationLoader::description_of)
            .map_err(K4QError::ConfigError)
        // .and_then(|config| config.try_into())
        // .map_err(ConfigurationLoader::description_of)
        // .map_err(K4QError::ConfigError)
    }
}


struct PartialConfig {
    config: Config,
}

impl PartialConfig {
    pub fn error_description_in_context<'a>(context: &'a str) -> (impl Fn(String) -> String + 'a)
    {
        move |description| format!(
            "Issue with property path: '{}' in the configuration file. The error: {}",
            context, description)
    }
}

impl ApplicationProperties for PartialConfig {
    fn properties_by(&self, prefix: &str) -> Result<Box<dyn ApplicationProperties>, K4QError> {
        let contextual_error = PartialConfig::error_description_in_context(prefix);

        self.config
            .get_table(prefix)
            .map(PartialConfigSource::new)
            .and_then(|c| Config::new().with_merged(c))
            .map(|c| PartialConfig { config: c })
            .map(ApplicationConfig::<PartialConfig>::new)
            .map(|c| Box::new(c) as Box<dyn ApplicationProperties>)
            .map_err(ConfigurationLoader::description_of)
            .map_err(contextual_error)
            .map_err(K4QError::ConfigError)
    }
}

impl CollectableProperties for PartialConfig {
    fn try_collect<'de, T>(self) -> Result<T, K4QError> where T: Sized + Deserialize<'de> {
        self.config
            .try_into()
            .map_err(ConfigurationLoader::description_of)
            .map_err(K4QError::ConfigError)
    }
}

#[derive(Debug, Clone)]
struct PartialConfigSource {
    props: HashMap<String, Value>,
}

impl PartialConfigSource {
    fn new(props: HashMap<String, Value>) -> PartialConfigSource {
        PartialConfigSource { props }
    }
}

impl Source for PartialConfigSource {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new(self.clone())
    }

    fn collect(&self) -> Result<HashMap<String, Value>, ConfigError> {
        Result::Ok(self.props.clone())
    }
}

impl ConfigurationLoader {
    fn description_of(err: ConfigError) -> String {
        match err {
            ConfigError::Frozen => String::from("Internal error - configuration is frozen and cannot be changed"),
            ConfigError::NotFound(msg) => format!("Check configuration file location. {}", msg),
            ConfigError::PathParse(err_kind) => {
                format!("Check supplied configuration file location path. The error: {}",
                        String::from(err_kind.description()))
            }
            ConfigError::FileParse { uri, cause } => match uri {
                None => format!("Correct configuration file. An error occurred during parsing: {}", cause),
                Some(file) => format!("Correct configuration file: {}. An error occurred during parsing: {}", file, cause)
            },
            ConfigError::Type { key, origin, unexpected, expected } => {
                let msg_key = key
                    .map(|k| format!(" Parsed key: {}", k))
                    .unwrap_or_else(|| String::from(""));
                let msg_config_location = origin
                    .map(|loc| format!(": {}", loc))
                    .unwrap_or_else(|| String::from(""));
                format!("Correct configuration file{}.{} Expected: {}, but encountered: {}",
                        msg_config_location,
                        msg_key,
                        expected,
                        unexpected)
            }
            ConfigError::Message(m) => format!("An issue with the configuration file: {}", m),
            ConfigError::Foreign(e) => {
                format!("An issue with the configuration file: {}", String::from(e.to_string()))
            }
        }
    }
}