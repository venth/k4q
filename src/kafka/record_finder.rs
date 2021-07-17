use crate::domain::criteria::Criteria;
use crate::domain::port::RecordFinder;
use crate::domain::record::Record;

pub fn new() -> Box<dyn RecordFinder> {
    Box::new(KafkaRecordFinder {})
}

impl RecordFinder for KafkaRecordFinder {
    fn find_by<'a>(&self, topics: &'a Vec<String>, criteria: &'a dyn Criteria) -> &'a dyn Iterator<Item=Record> {
        todo!()
    }
}

impl RecordFinder for Box<dyn RecordFinder> {
    fn find_by<'a>(&self, topics: &'a Vec<String>, criteria: &'a dyn Criteria) -> &'a dyn Iterator<Item=Record> {
        self.as_ref().find_by(&topics, criteria)
    }
}

impl RecordFinder for &dyn RecordFinder {
    fn find_by<'a>(&self, topics: &'a Vec<String>, criteria: &'a dyn Criteria) -> &'a dyn Iterator<Item=Record> {
        self.as_ref().find_by(&topics, criteria)
    }
}

struct KafkaRecordFinder {}