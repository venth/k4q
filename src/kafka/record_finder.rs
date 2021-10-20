use rxrust::observable;
use rxrust::observable::Observable;
use rxrust::ops::box_it::LocalBoxOp;
use shaku::Component;

use crate::domain::model::{Criteria, KeyValue, Offset, Partition, Payload, TopicName};
use crate::domain::model::Record;
use crate::domain::ports;

impl ports::RecordFinder for KafkaRecordFinder {
    fn find_by(&self, topic_name: TopicName) -> LocalBoxOp<Record, ()> {
        observable::repeat(
            move || Record::of(
                topic_name.clone(),
                KeyValue::from("key"),
                Partition::from(&0),
                Offset::from(&0),
                Payload::from("{}")),
        4280)
            .map(|f| f())
            .box_it()
    }
}


#[derive(Component)]
#[shaku(interface = ports::RecordFinder)]
pub struct KafkaRecordFinder {}