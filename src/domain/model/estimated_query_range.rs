use crate::domain::model::count::Count;
use crate::domain::model::TopicName;

pub struct EstimatedQueryRange {
    pub topic_name: TopicName,
    pub estimated_record_count: Count,
}

unsafe impl Send for EstimatedQueryRange {}
unsafe impl Sync for EstimatedQueryRange {}