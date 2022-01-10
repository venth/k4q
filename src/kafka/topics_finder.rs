use std::time::Duration;

use do_notation::m;
use futures::{stream, StreamExt};
use futures::stream::BoxStream;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::metadata::{Metadata, MetadataTopic};

use crate::domain::model;
use crate::domain::model::{K4fqError, Topic, TopicName};
use crate::domain::model::K4fqError::KafkaError;
use crate::domain::ports;
use crate::monads::Reader;
use crate::monads::ResultT;

pub struct KafkaTopicsFinder {
    consumer: StreamConsumer,
    timeout: Duration,
}

impl ports::TopicsFinder for KafkaTopicsFinder {
    fn find_by<'a>(&'a self, topics_matcher_type: &'a model::TopicsMatcherType) -> BoxStream<'a, Result<Topic, K4fqError>> {
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

type KafkaReader<'a, T> = Reader<'a, StreamConsumer, Result<T, K4fqError>>;
type KafkaReaderT<'a, T> = ResultT<'a, StreamConsumer,T, K4fqError>;

impl KafkaTopicsFinder {
    pub fn new(consumer: StreamConsumer) -> Self {
        Self { consumer, timeout: Duration::from_secs(2) }
    }

    fn topic_by(&self, topic_name: TopicName) -> Result<Topic, K4fqError> {
/*        let topic_metadata = m! {
            metadata <- ResultT::lift(self.fetch_metadata_for(&topic_name));
            let topic = Self::first_of(metadata.topics());

            topic.ok_or(K4fqError::KafkaError(format!("Cannot find topic: {:?}", topic_name)))
        };

        let topic = m! {
            metadata <- topic_metadata;
            partitions <- ResultT::lift(self.fetch_partitions_for(metadata));
            let a = Topic::new(topic_name, partitions);
            return Ok(a);
         };
        topic.value().apply(&self.consumer);
*/
    Err(K4fqError::NotSupported)
    }

    fn fetch_partitions_for(&self, topic: &MetadataTopic) -> KafkaReader<Vec<model::Partition>> {
        Reader::new(move |consumer: &StreamConsumer|
            (1..=5)
                .map(|id| stub_partition(id - 1, 0, 10))
                .map(Result::Ok)
                .collect::<Result<Vec<model::Partition>, K4fqError>>())
    }

    fn fetch_metadata_for(&self, topic_name: &TopicName) -> Reader<StreamConsumer, Result<Metadata, K4fqError>> {
        let topic = topic_name.clone();
        Reader::new(move |consumer: &StreamConsumer| consumer
            .client()
            .fetch_metadata(Some(topic.as_str()), self.timeout)
            .map_err(|e| K4fqError::KafkaError(e.to_string())))
    }

    fn first_of<A>(arr: &[A]) -> Option<&A> {
        arr.iter().next()
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
