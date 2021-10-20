use std::iter;

use shaku::Component;

use crate::domain::model::{KeyValue, Offset, Partition, Payload, TopicName};
use crate::domain::model::Record;
use crate::domain::ports;

impl ports::RecordFinder for KafkaRecordFinder {
    fn find_by<'a>(&self, topic_name: &'a TopicName) -> Box<dyn Iterator<Item=Record>> {
        let topic_name = topic_name.clone();
        Box::new(iter::repeat(move || Record::of(
            topic_name,
            KeyValue::from("key"),
            Partition::from(&0),
            Offset::from(&0),
            Payload::from("{}"),
        ))
            .take(4280)
            .map(|f| f()))
    }
}


#[derive(Component)]
#[shaku(interface = ports::RecordFinder)]
pub struct KafkaRecordFinder {}