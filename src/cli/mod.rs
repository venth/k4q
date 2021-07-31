use std::sync::Arc;

use shaku::module;

use crate::cli::action_selector::key_equals_value;
use crate::domain;

mod action_recognizer;
mod cli_parser;
mod action_selector;
mod command_recognizer;

module! {
    pub CliModule: domain::CliModule {
        components = [
            action_recognizer::CliActionRecognizer,
            cli_parser::ClapCliParserFactory,
            key_equals_value::Matcher,
            command_recognizer::CliCommandRecognizer,
        ],
        providers = [],
    }
}

pub fn module() -> Arc<dyn domain::CliModule> {
    Arc::new(CliModule::builder()
        .build())
}
