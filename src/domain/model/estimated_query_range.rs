use crate::domain::model::count::Count;
use crate::domain::model::TopicName;

pub struct EstimatedQueryRange {
    pub topic_name: TopicName,
    pub estimated_record_count: Count,
}