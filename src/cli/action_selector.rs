use std::sync::Arc;

use clap::ArgMatches;
use shaku::Interface;

use crate::domain::action;
use crate::domain::action::Action;
use crate::domain::service::ActionFactory;

pub mod key_equals_value;

pub trait ActionSelector: Interface {
    fn select_by(&self, matched: &ArgMatches) -> Option<Box<dyn action::Action>>;
}
