use std::path::PathBuf;

use home::home_dir;

use crate::domain::model::criteria::Criteria;
use crate::domain::model::topics_matcher_type::TopicsMatcherType;

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
    QueryByKey(Box<ConfigurationSetup>, TopicsMatcherType, Box<dyn Criteria>),
    CommandNotRecognized,
}