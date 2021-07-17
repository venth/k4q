use std::borrow::Borrow;
use std::ops::Deref;

use crate::domain::action::{Action, ActionFactory};
use crate::domain::criteria::Criteria;
use crate::domain::port::RecordFinder;
use crate::domain::record::{KeyValue, Record};

pub fn create_action_factory(record_finder: Box<dyn RecordFinder>) -> Box<dyn ActionFactory> {
    Box::new(
        QueryActionFactory{ record_finder }
    )
}

impl ActionFactory for QueryActionFactory {
    fn using(&self, criteria: Box<dyn Criteria>, topics: Vec<&str>) -> Box<dyn Action> {
        Box::from(QueryAction {
            record_finder: Box::new(self.record_finder.deref()),
            criteria: Box::from(criteria),
            topics: topics.iter().map(|e| String::from(*e)).collect(),
        })
    }
}

struct QueryActionFactory {
    record_finder: Box<dyn RecordFinder>,
}

struct QueryAction {
    record_finder: Box<dyn RecordFinder>,
    criteria: Box<dyn Criteria>,
    topics: Vec<String>,
}

impl Action for QueryAction {
    fn execute(&self) -> Box<dyn Iterator<Item=Record>> {
        // let _ = self.criteria.test(&Record::default());
        Box::new(std::iter::empty::<Record>())
    }
}