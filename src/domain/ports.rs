use shaku::Interface;

use crate::domain::model::{Command, TopicName, TopicsMatcherType};
use crate::domain::model::Criteria;
use crate::domain::model::Record;

pub trait RecordFinder: Interface {
    fn find_by<'a>(&self,
                   topic_name: TopicName,
                   criteria: &'a dyn Criteria) -> Box<dyn Iterator<Item=Record>>;
}

pub trait CommandRecognizer: Interface {
    fn recognize(&self, args: &Vec<&str>) -> Option<Command>;
}

pub trait ProgressNotifier: Interface {
    fn notify(&self, message: &str);
}

pub trait TopicsFinder: Interface {
    fn find_by<'a>(&self, topics_matcher_type: &'a TopicsMatcherType) -> Box<dyn Iterator<Item=TopicName> + 'a>;
}
