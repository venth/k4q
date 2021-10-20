use std::sync::Arc;

use rxrust::ops::box_it::LocalBoxOp;
use shaku::Interface;

use crate::domain::model::{Command, TopicName, TopicsMatcherType};
use crate::domain::model::Criteria;
use crate::domain::model::Progress;
use crate::domain::model::Record;

pub trait RecordFinder: Interface {
    fn find_by(&self, topic_name: TopicName) -> LocalBoxOp<Record, ()>;
}

pub trait CommandRecognizer: Interface {
    fn recognize(&self, args: &Vec<&str>) -> Option<Command>;
}

pub trait ProgressNotifier: Interface {
    fn notify(&self, message: &str);
    fn start(&self) -> Arc<dyn Progress>;
}

pub trait TopicsFinder: Interface {
    fn find_by<'a>(&self,
               topics_matcher_type: &'a TopicsMatcherType)
               -> LocalBoxOp<'a, TopicName, ()>;
}

