use clap::ArgMatches;

use crate::domain::action;
use crate::domain::action::Action;

pub mod key_equlas_value;

pub trait QueryMatcher {
    fn matches(&self, matched: &ArgMatches) -> Option<Box<dyn action::Action>>;
}

impl QueryMatcher for Box<dyn QueryMatcher> {
    fn matches(&self, matched: &ArgMatches) -> Option<Box<dyn Action>> {
        self.as_ref().matches(&matched)
    }
}