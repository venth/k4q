use crate::domain::model::partition::Partition;
use crate::domain::model::TopicName;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Topic {
    pub topic_name: TopicName,
    pub partitions: Vec<Partition>,
}

impl Topic {
    pub fn new(topic_name: TopicName, partitions: Vec<Partition>) -> Self {
        Topic {
            topic_name,
            partitions
        }
    }

    pub fn record_count(&self) -> i64 {
        self.partitions.iter()
            .map(Partition::record_count)
            .sum()
    }
}