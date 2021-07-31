use std::sync::Arc;

use shaku::{HasComponent, Interface, ModuleInterface};
use shaku::module;

use crate::domain;
use crate::domain::action::Action;
use crate::domain::criteria::Criteria;
use crate::domain::query::QueryActionFactory;

pub trait App: Interface {
    fn run<'a>(&self, args: &'a Vec<&'a str>);
}

pub trait ActionFactory: Interface {
    fn create(&self, criteria: Box<dyn Criteria>, topics: Vec<&str>) -> Box<dyn Action>;
}
