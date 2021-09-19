use crate::domain::model::{KeyValue, Record};

pub trait Criteria {
    fn test(&self, rec: &Record) -> bool;
}

pub fn key_equals_value(value: KeyValue) -> Box<dyn Criteria> {
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
    key: KeyValue,
}

