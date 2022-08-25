use std::sync::Arc;
use std::time::Duration;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::error::KafkaResult;
use rdkafka::metadata::Metadata;

pub struct TimeoutAwareStreamConsumer {
    consumer: Arc<StreamConsumer>,
    timeout: Duration,
}

pub fn new(consumer: StreamConsumer, timeout: Duration) -> TimeoutAwareStreamConsumer {
    TimeoutAwareStreamConsumer { consumer: Arc::new(consumer), timeout }
}

impl TimeoutAwareStreamConsumer {
    pub fn fetch_watermarks(&self, topic: &str, partition: i32) -> KafkaResult<(i64, i64)> {
        self.consumer.clone()
            .fetch_watermarks(topic, partition, self.timeout)
    }

    pub fn fetch_metadata(&self, topic: Option<&str>) -> KafkaResult<Metadata> {
        self.consumer.clone()
            .fetch_metadata(topic, self.timeout)
    }
}