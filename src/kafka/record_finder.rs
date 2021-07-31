use shaku::Component;

use crate::domain::criteria::Criteria;
use crate::domain::port::RecordFinder;
use crate::domain::record::Record;

impl RecordFinder for KafkaRecordFinder {
    fn find_by<'a>(&self, topics: &'a Vec<String>, criteria: &'a dyn Criteria) -> &'a dyn Iterator<Item=Record> {
        todo!()
    }
}


#[derive(Component)]
#[shaku(interface = RecordFinder)]
pub struct KafkaRecordFinder {}