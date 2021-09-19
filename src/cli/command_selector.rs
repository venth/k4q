use clap::ArgMatches;
use shaku::Interface;

use crate::domain::model::Command;

pub mod key_equals_value;

pub trait CommandSelector: Interface {
    fn select_by<'a>(&self, matched: &'a ArgMatches) -> Option<Command<'a>>;
}
