use shaku::Component;

use crate::domain::model::{Criteria, KeyValue, Offset, Partition, Payload, TopicName};
use crate::domain::model::Record;
use crate::domain::ports::RecordFinder;

impl RecordFinder for KafkaRecordFinder {
    fn find_by<'a>(&self, topics: Vec<&str>, criteria: &'a dyn Criteria) -> Box<dyn Iterator<Item=Record>> {
        Box::new(vec![
            Record::of(
                TopicName::from("topic name"),
                KeyValue::from("key"),
                Partition::from(&0),
                Offset::from(&0),
                Payload::from("{}"),
            )
        ].into_iter())
    }
}


#[derive(Component)]
#[shaku(interface = RecordFinder)]
pub struct KafkaRecordFinder {}