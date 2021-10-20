use std::sync::Arc;

use shaku::Interface;

use crate::domain::model::{Command, TopicsMatcherType};
use crate::domain::model::Progress;
use crate::domain::model::Record;
use crate::domain::model::TopicName;

pub trait RecordFinder: Interface {
    fn find_by<'a>(&self,
                   topic_name: &'a TopicName) -> Box<dyn Iterator<Item=Record>>;
}

pub trait CommandRecognizer: Interface {
    fn recognize(&self, args: &Vec<&str>) -> Option<Command>;
}

pub trait ProgressNotifier: Interface {
    fn notify(&self, message: &str);
    fn start(&self) -> Arc<dyn Progress>;
}

pub trait TopicsFinder: Interface {
    fn find_by<'a>(&self, topics_matcher_type: &'a TopicsMatcherType) -> Box<dyn Iterator<Item=TopicName> + 'a>;
}
