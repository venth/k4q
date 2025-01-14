use crate::domain::model;
use crate::domain::model::{K4fqError, Partition};
use crate::kafka::kafka_reader::TimeoutAwareKafkaReader;

pub fn new<'a>(_: model::TopicName) -> TimeoutAwareKafkaReader<'a, Vec<Partition>> {
    /*let metadata_reader = metadata_reader::new(topic_name);

    metadata_reader
        .map(|r| r.map(|metadata| metadata.))*/
    TimeoutAwareKafkaReader::new(move |_| Result::Err(K4fqError::NotSupported))
}
