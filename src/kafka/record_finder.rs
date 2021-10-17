use std::iter;

use shaku::Component;

use crate::domain::model::{Criteria, KeyValue, Offset, Partition, Payload, TopicName};
use crate::domain::model::Record;
use crate::domain::ports::RecordFinder;

impl RecordFinder for KafkaRecordFinder {
    fn find_by<'a>(&self, topic_name: TopicName, criteria: &'a dyn Criteria) -> Box<dyn Iterator<Item=Record>> {
        Box::new(iter::repeat(|| Record::of(
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
#[shaku(interface = RecordFinder)]
pub struct KafkaRecordFinder {}