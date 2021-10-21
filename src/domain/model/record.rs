use serde;

use crate::domain::model::partition_id::PartitionId;
use crate::domain::model::topic_name::TopicName;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Record {
    topic_name: TopicName,
    key: KeyValue,
    partition: PartitionId,
    offset: Offset,
    payload: Payload,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct KeyValue {
    value: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Offset {
    value: i64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Payload {
    value: String,
}

impl From<&str> for KeyValue {
    fn from(value: &str) -> Self {
        Self { value: String::from(value) }
    }
}

impl From<&str> for Payload {
    fn from(value: &str) -> Self {
        Self { value: String::from(value) }
    }
}

impl From<i64> for Offset {
    fn from(value: i64) -> Self {
        Self { value }
    }
}

impl Record {
    pub fn of(
        topic_name: TopicName,
        key: KeyValue,
        partition: PartitionId,
        offset: Offset,
        payload: Payload) -> Self {
        Self {
            topic_name: TopicName::from(topic_name),
            key: KeyValue::from(key),
            partition: PartitionId::from(partition),
            offset: Offset::from(offset),
            payload: Payload::from(payload),
        }
    }
}