use std::path::PathBuf;

use crate::domain::model::criteria::Criteria;
use crate::domain::model::topics_matcher_type::TopicsMatcherType;

pub struct ConfigurationSetup {
    pub location: PathBuf,
}

pub enum Command {
    QueryByKey(Box<Option<ConfigurationSetup>>, TopicsMatcherType, Box<dyn Criteria>),
    CommandNotRecognized,
}