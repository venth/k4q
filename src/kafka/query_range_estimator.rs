use rdkafka::consumer::StreamConsumer;

use crate::domain::model::{Count, EstimatedQueryRange, QueryRange, Topic};
use crate::domain::ports;

impl ports::QueryRangeEstimator for KafkaQueryRangeEstimator {
    fn estimate(&self, topic: &Topic, query_range: &QueryRange) -> EstimatedQueryRange {
        match query_range {
            QueryRange::Whole => {
                EstimatedQueryRange {
                    topic_name: topic.topic_name.clone(),
                    estimated_record_count: topic.record_count() as Count,
                }
            }
        }
    }
}

impl KafkaQueryRangeEstimator {
    pub fn new(consumer: StreamConsumer) -> Self {
        Self { consumer }
    }
}

pub struct KafkaQueryRangeEstimator {
    consumer: StreamConsumer,
}
