use shaku::Component;

use crate::domain::model;
use crate::domain::ports;

#[derive(Component)]
#[shaku(interface = ports::TopicsFinder)]
pub struct KafkaTopicsFinder {}

impl ports::TopicsFinder for KafkaTopicsFinder {
    fn find_by<'a>(&self, topics_matcher_type: &'a model::TopicsMatcherType) -> Box<dyn Iterator<Item=model::TopicName> + 'a> {
        match topics_matcher_type {
            model::TopicsMatcherType::DIRECT(topics) => { Box::new(topics.iter().map(model::TopicName::from)) }
            _ => { Box::new(std::iter::empty::<model::TopicName>()) }
        }
    }
}
