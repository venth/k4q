#[derive(Debug)]
pub struct Record {
    topic: TopicName,
    key: KeyValue,
    partition: Partition,
    offset: Offset,
    payload: Payload,
}

#[derive(Debug)]
pub struct KeyValue {
    value: String,
}

#[derive(Debug)]
pub struct Partition {
    value: i32,
}

#[derive(Debug)]
pub struct Offset {
    value: i64,
}

#[derive(Debug)]
pub struct TopicName {
    value: String,
}

#[derive(Debug)]
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

impl From<&str> for TopicName {
    fn from(value: &str) -> Self {
        Self { value: String::from(value) }
    }
}

impl From<&i32> for Partition {
    fn from(value: &i32) -> Self {
        Self { value: *value }
    }
}

impl From<&i64> for Offset {
    fn from(value: &i64) -> Self {
        Self { value: *value }
    }
}

impl Record {
    fn new(
        topic: TopicName,
        key: KeyValue,
        partition: Partition,
        offset: Offset,
        payload: Payload) -> Self {
        Self {
            topic: TopicName::from(topic),
            key: KeyValue::from(key),
            partition: Partition::from(partition),
            offset: Offset::from(offset),
            payload: Payload::from(payload),
        }
    }
}