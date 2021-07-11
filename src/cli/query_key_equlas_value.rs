use std::iter::FromIterator;

use clap::ArgMatches;

use domain::record::KeyValue;

use crate::domain;
use crate::domain::query;

pub fn matches(matched: &ArgMatches) -> Option<impl query::Action> {
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
        .map(|(op, v)| query::key_equals_value(KeyValue::from(v)))
        .zip(query_topics)
        .map(|(crit, topics)| query::using(crit, topics))
}
