use crate::domain::model::Record;
use crate::domain::model::record_key::RecordKey;

pub trait Criteria: Sync + Send {
    fn test(&self, rec: &Record) -> bool;
}

pub fn key_equals_value(value: RecordKey) -> Box<dyn Criteria> {
    Box::new(CriteriaKeyEqualsValue { key: value })
}

impl Criteria for Box<dyn Criteria> {
    fn test(&self, rec: &Record) -> bool {
        self.as_ref().test(&rec)
    }
}

impl Criteria for CriteriaKeyEqualsValue {
    fn test(&self, rec: &Record) -> bool {
        false
    }
}

#[derive(Debug)]
struct CriteriaKeyEqualsValue {
    key: RecordKey,
}

