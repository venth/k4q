use std::collections::HashMap;

use config::{Config, ConfigError, Source, Value};

pub struct ConfigurationLoader {}


struct PartialConfig {
    config: Config,
}

impl PartialConfig {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn error_description_in_context<'a>(context: &'a str) -> (impl Fn(String) -> String + 'a)
    {
        move |description| format!(
            "Issue with property path: '{}' in the configuration file. The error: {}",
            context, description)
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
