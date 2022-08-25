use crate::domain::model;
use crate::domain::model::K4fqError;
use crate::kafka::kafka_reader::TimeoutAwareKafkaReader;
use crate::kafka::timeout_aware_stream_consumer::TimeoutAwareStreamConsumer;
use crate::monads::Reader;

pub fn new<'a>(topic_name: model::TopicName,
               partition_id: model::PartitionId) -> TimeoutAwareKafkaReader<'a, model::Partition> {
    Reader::new(
        move |consumer: &TimeoutAwareStreamConsumer| consumer
            .fetch_watermarks(topic_name.as_str(), partition_id.value())
            .map_err(|e| K4fqError::KafkaError(e.to_string()))
            .map(|(low, high)| (model::Watermark::from(low), model::Watermark::from(high)))
            .map(|(low, high)| model::Partition::new(partition_id, low, high)))
}
