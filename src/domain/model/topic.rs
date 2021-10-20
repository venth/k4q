use crate::domain::model::TopicName;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Topic {
    topic_name: TopicName,
    
}
