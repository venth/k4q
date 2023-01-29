use crate::domain::model;
use crate::domain::model::Partition;
use crate::kafka::kafka_reader::TimeoutAwareKafkaReader;
use crate::kafka::{metadata_reader, partition_reader};

pub fn new<'a>(topic_name: model::TopicName) -> TimeoutAwareKafkaReader<'a, Vec<Partition>> {
    let metadata_reader = metadata_reader::new(topic_name);

    metadata_reader.map(|m| Vec::new())
        .map(Ok)
}
