use std::pin::Pin;

use futures::{Stream, StreamExt};
use futures::stream;
use shaku::Component;

use crate::domain::model;
use crate::domain::ports;

#[derive(Component)]
#[shaku(interface = ports::TopicsFinder)]
pub struct KafkaTopicsFinder {}

impl ports::TopicsFinder for KafkaTopicsFinder {
    fn find_by<'a>(&self, topics_matcher_type: &'a model::TopicsMatcherType) -> Pin<Box<dyn Stream<Item=model::TopicName> + 'a>> {
        match topics_matcher_type {
            model::TopicsMatcherType::DIRECT(topics) => {
                stream::iter(topics)
                    .map(model::TopicName::from).boxed()
            }
            _ => { Box::pin(stream::empty::<model::TopicName>()) }
        }
    }
}
