use std::iter::FromIterator;

use clap::ArgMatches;

use domain::record::KeyValue;

use crate::cli::query_matcher::QueryMatcher;
use crate::domain;
use crate::domain::{action, criteria, query};
use crate::domain::action::{Action, ActionFactory};

pub fn new(action_factory: Box<dyn ActionFactory>) -> Box<dyn QueryMatcher> {
    Box::new(KeyEqualsValueMatcher{ action_factory })
}

impl QueryMatcher for KeyEqualsValueMatcher {
    fn matches(&self, matched: &ArgMatches) -> Option<Box<dyn Action>> {
        let query_topics = matched
            .subcommand_matches("query")
            .and_then(|m| m.values_of("topics"))
            .map(|topics| Vec::from_iter(topics));

        let query_key_criteria = matched
            .subcommand_matches("query")
            .and_then(|m| m.subcommand_matches("key"))
            .and_then(|m| m.value_of("criteria"));

        let query_key_value = matched
            .subcommand_matches("query")
            .and_then(|m| m.subcommand_matches("key"))
            .and_then(|m| m.value_of("keyValue"));

        query_key_criteria
            .zip(query_key_value)
            .map(|(op, v)| criteria::key_equals_value(KeyValue::from(v)))
            .zip(query_topics)
            .map(|(crit, topics)| self.action_factory.using(crit, topics))
    }
}

struct KeyEqualsValueMatcher {
    action_factory: Box<dyn ActionFactory>
}
