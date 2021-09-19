use shaku::Interface;

use crate::domain::action::Action;
use crate::domain::model::Criteria;

pub trait App: Interface {
    fn run<'a>(&self, args: &'a Vec<&'a str>);
}

pub trait ActionFactory: Interface {
    fn create(&self, criteria: Box<dyn Criteria>, topics: Vec<&str>) -> Box<dyn Action>;
}
