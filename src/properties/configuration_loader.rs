use std::path::Path;

use config::{Config, ConfigError};

use crate::domain::model::{K4QError, Properties};
use crate::domain::ports;

#[derive(shaku::Component)]
#[shaku(interface = ports::PropertiesSource)]
pub struct ConfigurationLoader {}

impl ports::PropertiesSource for ConfigurationLoader {
    fn load(&self, config_location: &Path) -> Result<Properties, K4QError> {
        Config::default()
            .with_merged(config::File::with_name(config_location.to_str().unwrap()))
            .and_then(|config| config.try_into())
            .map_err(ConfigurationLoader::description_of)
            .map_err(K4QError::ConfigError)
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