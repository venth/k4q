use crate::domain::model;
use crate::domain::model::Partition;
use crate::kafka::kafka_reader::TimeoutAwareKafkaReader;
use crate::kafka::{metadata_reader, partition_reader};

pub fn new(topic_name: model::TopicName) -> TimeoutAwareKafkaReader<Vec<Partition>> {
    let metadata_reader = metadata_reader::new(topic_name);

    metadata_reader
        .map(|r| r.map(|metadata| metadata.))
}