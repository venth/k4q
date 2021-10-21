use std::pin::Pin;
use std::sync::Arc;

use futures::Stream;
use shaku::Interface;

use crate::domain::model::{Command, Topic, TopicsMatcherType};
use crate::domain::model::Progress;
use crate::domain::model::Record;
use crate::domain::model::TopicName;

pub trait RecordFinder: Interface {
    fn find_by<'a>(&self, topic_name: &'a TopicName) -> Pin<Box<dyn Stream<Item=Record>>>;
}

pub trait CommandRecognizer: Interface {
    fn recognize(&self, args: &Vec<&str>) -> Option<Command>;
}

pub trait ProgressNotifier: Interface {
    fn notify(&self, message: &str);
    fn start(&self, estimated_max_size: &i64) -> Arc<dyn Progress>;
}

pub trait TopicsFinder: Interface {
    fn find_by<'a>(&self, topics_matcher_type: &'a TopicsMatcherType) -> Pin<Box<dyn Stream<Item=Topic> + 'a>>;
}
