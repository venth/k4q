use shaku;

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

#[derive(shaku::Component)]
#[shaku(interface = ports::QueryRangeEstimator)]
pub struct KafkaQueryRangeEstimator {}
