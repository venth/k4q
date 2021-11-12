use std::pin::Pin;

use futures::{stream, Stream, StreamExt};
use shaku::Component;

use crate::domain::model;
use crate::domain::ports;

#[derive(Component)]
#[shaku(interface = ports::TopicsFinder)]
pub struct KafkaTopicsFinder {}

impl ports::TopicsFinder for KafkaTopicsFinder {
    fn find_by<'a>(&self, topics_matcher_type: &'a model::TopicsMatcherType) -> Pin<Box<dyn Stream<Item=model::Topic> + 'a>> {
        match topics_matcher_type {
            model::TopicsMatcherType::DIRECT(topics) => {
                stream::iter(topics)
                    .map(model::TopicName::from)
                    .map(move |topic_name| model::Topic::new(topic_name, stub_partitions(1)))
                    .boxed()
            }
        }
    }
}

fn stub_partition(id: i32, low: i64, high: i64) -> model::Partition {
    model::Partition::new(
        model::PartitionId::from(id),
        model::Watermark::from(low),
        model::Watermark::from(high))
}

fn stub_partitions(number_of_partitions: i32) -> Vec<model::Partition> {
    (1..number_of_partitions)
        .map(|id| stub_partition(id - 1, 0, 10))
        .collect::<Vec<model::Partition>>()
}
