use std::time::Duration;
use rdkafka::consumer::StreamConsumer;
use crate::domain::model::{K4fqError, TopicName};
use crate::kafka::timeout_aware_stream_consumer::TimeoutAwareStreamConsumer;
use crate::monads::Reader;

pub type KafkaReader<'a, T> = Reader<'a, StreamConsumer, Result<T, K4fqError>>;
pub type TimeoutAwareKafkaReader<'a, T> = Reader<'a, TimeoutAwareStreamConsumer, Result<T, K4fqError>>;

pub type SpecificTopicReader<'a, T> = Reader<'a, TopicName, TimeoutAwareKafkaReader<'a, T>>;