use clap::ArgMatches;
use shaku::Interface;

use crate::domain::action;

pub mod key_equals_value;

pub trait ActionSelector: Interface {
    fn select_by(&self, matched: &ArgMatches) -> Option<Box<dyn action::Action>>;
}
