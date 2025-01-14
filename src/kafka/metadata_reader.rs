use crate::domain::model;
use crate::domain::model::K4fqError;
use crate::kafka::kafka_reader::TimeoutAwareKafkaReader;
use crate::monads::Reader;
use rdkafka::metadata::Metadata;

pub fn new<'a>(topic_name: model::TopicName) -> TimeoutAwareKafkaReader<'a, Metadata> {
    /*Reader::new(move |consumer: &TimeoutAwareStreamConsumer| consumer
        .fetch_metadata(Some(topic_name.as_str()))
        .map_err(|e| model::K4fqError::KafkaError(e.to_string())))*/
    Reader::new(move |c| Result::Err(K4fqError::NotSupported))
}
