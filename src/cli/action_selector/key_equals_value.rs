use std::iter::FromIterator;

use clap::ArgMatches;
use shaku;
use shaku::Component;

use crate::cli::action_selector::ActionSelector;
use crate::domain::action::Action;

impl ActionSelector for Matcher {
    fn select_by(&self, matched: &ArgMatches) -> Option<Box<dyn Action>> {
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

        /*        query_key_criteria
                    .zip(query_key_value)
                    .map(|(op, v)| criteria::key_equals_value(KeyValue::from(v)))
                    .zip(query_topics)
                    .map(|(crit, topics)| self.action_factory.create(crit, topics))
        */
        Option::None
    }
}

#[derive(Component)]
#[shaku(interface = ActionSelector)]
pub struct Matcher {}
