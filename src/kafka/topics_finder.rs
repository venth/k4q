use std::time::Duration;

use futures::{stream, StreamExt};
use futures::stream::BoxStream;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::metadata::MetadataTopic;

use crate::domain::model;
use crate::domain::model::{K4QError, Topic, TopicName};
use crate::domain::ports;

pub struct KafkaTopicsFinder {
    consumer: StreamConsumer,
    timeout: Duration,
}

impl ports::TopicsFinder for KafkaTopicsFinder {
    fn find_by<'a>(&'a self, topics_matcher_type: &'a model::TopicsMatcherType) -> BoxStream<'a, Result<Topic, K4QError>> {
        match topics_matcher_type {
            model::TopicsMatcherType::DIRECT(topics) => {
                stream::iter(topics)
                    .map(model::TopicName::from)
                    .map(move |topic_name| self.topic_by(topic_name))
                    .boxed()
            }
        }
    }
}

impl KafkaTopicsFinder {
    pub fn new(consumer: StreamConsumer) -> Self {
        Self { consumer, timeout: Duration::from_secs(2) }
    }

    fn topic_by(&self, topic_name: TopicName) -> Result<Topic, K4QError> {
        self.consumer
            .client()
            .fetch_metadata(Some(topic_name.as_str()), self.timeout)
            .map_err(|e| K4QError::KafkaError(e.to_string()))
            .and_then(|r| r.topics().first().map(MetadataTopic::name).map(ToString::to_string).ok_or(K4QError::KafkaError("Topic not found".to_string())))
            .map(|t| model::Topic::new(TopicName::from(t.as_str()), stub_partitions(1)))
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
