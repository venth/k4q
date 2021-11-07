use std::iter::FromIterator;

use clap::ArgMatches;
use shaku;
use shaku::Component;

use crate::cli::command_selector::CommandSelector;
use crate::domain::model::{Command, TopicsMatcherType};
use crate::domain::model;
use crate::domain::model::RecordKey;

impl CommandSelector for Matcher {
    fn select_by<'a>(&self, matched: ArgMatches) -> Option<Command> {
        let query_topics: Option<Vec<String>> = matched
            .subcommand_matches("query")
            .and_then(|m| m.values_of("topics"))
            .map(|topics| Vec::from_iter(topics))
            .map(Vec::into_iter)
            .map(|topics| topics.map(ToString::to_string))
            .map(|topics| topics.collect());

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
            .map(|(op, v)| model::key_equals_value(RecordKey::from(v)))
            .zip(query_topics)
            .map(|(crit, topics)| Command::QueryByKey(
                TopicsMatcherType::DIRECT(topics),
                crit))
    }
}

#[derive(Component)]
#[shaku(interface = CommandSelector)]
pub struct Matcher {}
