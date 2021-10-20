use rxrust::ops::box_it::LocalBoxOp;
use rxrust::prelude as observable;
use rxrust::prelude::Observable;
use shaku::Component;

use crate::domain::model;
use crate::domain::model::{TopicName, TopicsMatcherType};
use crate::domain::ports;

#[derive(Component)]
#[shaku(interface = ports::TopicsFinder)]
pub struct KafkaTopicsFinder {}

impl ports::TopicsFinder for KafkaTopicsFinder {
    fn find_by<'a>(&self,
               topics_matcher_type: &'a model::TopicsMatcherType)
               -> LocalBoxOp<'a, TopicName, ()> {
        KafkaTopicsFinder::match_by(&topics_matcher_type)
    }
}

impl KafkaTopicsFinder {
    fn match_by(topics_matcher_type: &TopicsMatcherType) -> LocalBoxOp<TopicName, ()> {
        match topics_matcher_type {
            model::TopicsMatcherType::DIRECT(topics) =>
                observable::from_iter(topics.iter())
                    .map(model::TopicName::from).box_it(),

            _ => observable::empty::<model::TopicName>().box_it()
        }
    }
}
