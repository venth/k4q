use shaku::Component;

use crate::domain::model::Criteria;
use crate::domain::model::Record;
use crate::domain::ports::RecordFinder;

impl RecordFinder for KafkaRecordFinder {
    fn find_by<'a>(&self, topics: &'a Vec<String>, criteria: &'a dyn Criteria) -> &'a dyn Iterator<Item=Record> {
        todo!()
    }
}


#[derive(Component)]
#[shaku(interface = RecordFinder)]
pub struct KafkaRecordFinder {}