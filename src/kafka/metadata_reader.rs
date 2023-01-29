use rdkafka::metadata::Metadata;
use crate::domain::model;
use crate::kafka::kafka_reader::TimeoutAwareKafkaReader;
use crate::kafka::timeout_aware_stream_consumer::TimeoutAwareStreamConsumer;
use crate::monads::Reader;

pub fn new<'a>(topic_name: model::TopicName) -> TimeoutAwareKafkaReader<'a, Metadata> {
    Reader::new(move |consumer: &TimeoutAwareStreamConsumer| consumer
        .fetch_metadata(Some(topic_name.as_str()))
        .map_err(|e| model::K4fqError::KafkaError(e.to_string())))
}
