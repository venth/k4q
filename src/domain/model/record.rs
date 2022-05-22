use serde;

use crate::domain::model::offset::Offset;
use crate::domain::model::partition_id::PartitionId;
use crate::domain::model::record_key::RecordKey;
use crate::domain::model::topic_name::TopicName;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Record {
    topic_name: TopicName,
    record_key: RecordKey,
    partition_id: PartitionId,
    offset: Offset,
    payload: Payload,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
#[serde(into="String")]
pub struct Payload {
    value: String,
}

impl From<Payload> for String {
    fn from(payload: Payload) -> Self {
        payload.value
    }
}

impl From<&str> for Payload {
    fn from(value: &str) -> Self {
        Self { value: String::from(value) }
    }
}

impl Record {
    pub fn of(
        topic_name: TopicName,
        record_key: RecordKey,
        partition_id: PartitionId,
        offset: Offset,
        payload: Payload) -> Self {
        Self {
            topic_name: TopicName::from(topic_name),
            record_key: RecordKey::from(record_key),
            partition_id: PartitionId::from(partition_id),
            offset: Offset::from(offset),
            payload: Payload::from(payload),
        }
    }
}