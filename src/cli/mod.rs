use std::sync::Arc;

use shaku::module;

use crate::domain;

mod action_recognizer;

module! {
    pub CliModule: domain::CliModule {
        components = [action_recognizer::CliActionRecognizer],
        providers = [],
    }
}

pub fn cli_module() -> Arc<dyn domain::CliModule> {
    Arc::new(CliModule::builder()
        .build())
}