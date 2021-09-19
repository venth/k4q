use shaku::Interface;

use crate::domain::model::Command;
use crate::domain::model::Criteria;
use crate::domain::model::Record;

pub trait RecordFinder: Interface {
    fn find_by<'a>(&self, topics: &'a Vec<String>,
                   criteria: &'a dyn Criteria) -> &'a dyn Iterator<Item=Record>;
}

pub trait CommandRecognizer: Interface {
    fn recognize(&self, args: &Vec<&str>) -> Option<Command>;
}

pub trait ErrorNotifier: Interface {
    fn notify(&self, message: &str);
}