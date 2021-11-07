use std::sync::Arc;

use shaku::module;

use crate::cli::command_selector::key_equals_value;
use crate::domain;

mod cli_parser;
mod command_selector;
mod command_recognizer;
mod properties_location_provider;

module! {
    pub CliModule: domain::CliModule {
        components = [
            cli_parser::ClapCliParserFactory,
            key_equals_value::Matcher,
            command_recognizer::CliCommandRecognizer,
            properties_location_provider::CliPropertiesLocationProvider,
        ],
        providers = [],
    }
}

pub fn module() -> Arc<dyn domain::CliModule> {
    Arc::new(CliModule::builder()
        .build())
}
