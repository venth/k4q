use std::iter;

use rdkafka::consumer::StreamConsumer;

use crate::domain::model::Offset;
use crate::domain::model::PartitionId;
use crate::domain::model::Payload;
use crate::domain::model::Record;
use crate::domain::model::RecordKey;
use crate::domain::model::TopicName;
use crate::domain::ports;

impl ports::RecordFinder for KafkaRecordFinder {
    fn find_by<'a>(&self, topic_name: &'a TopicName) -> Box<dyn Iterator<Item=Record>> {
        let topic_name = topic_name.clone();
        Box::new(iter::repeat(move ||
            Record::of(
                topic_name,
                RecordKey::from("key"),
                PartitionId::from(0),
                Offset::from(0),
                Payload::from("{}"),
            ))
            .take(10)
            .map(|f| f()))
    }
}


impl KafkaRecordFinder {
    pub fn new(consumer: StreamConsumer) -> Self {
        Self { consumer }
    }
}

pub struct KafkaRecordFinder {
    consumer: StreamConsumer,
}