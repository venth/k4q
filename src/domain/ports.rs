use std::sync::Arc;

use shaku::Interface;

use crate::domain::action::Action;
use crate::domain::command::Command;
use crate::domain::criteria::Criteria;
use crate::domain::record::Record;
use crate::domain::service::ActionFactory;

pub trait RecordFinder: Interface {
    fn find_by<'a>(&self, topics: &'a Vec<String>,
                   criteria: &'a dyn Criteria) -> &'a dyn Iterator<Item=Record>;
}

pub trait ActionRecognizer: Interface {
    fn recognize<'a>(&self,
                     args: &'a Vec<&'a str>) -> Option<Box<dyn Action + 'a>>;
}

pub trait CommandRecognizer: Interface {
    fn recognize(&self, args: &Vec<&str>) -> Option<Command>;
}

pub trait ErrorNotifier: Interface {
    fn notify(&self, message: &str);
}