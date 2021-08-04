use std::path::PathBuf;

use home::home_dir;

pub struct ConfigurationSetup {
    location: Option<PathBuf>,
    profile_name: String,
}

impl Default for ConfigurationSetup {
    fn default() -> Self {
        ConfigurationSetup {
            location: home_dir(),
            profile_name: "default".into(),
        }
    }
}

pub enum Command {
    QUERY(Box<ConfigurationSetup>),
}