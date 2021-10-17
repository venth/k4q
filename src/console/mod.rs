use std::sync::Arc;

use indicatif::{MultiProgress, ProgressStyle};
use shaku::module;

use crate::domain;

mod progress_notifier;

module! {
    pub ConsoleModule: domain::ConsoleModule {
        components = [
            progress_notifier::ConsoleErrorNotifier,
        ],
        providers = [],
    }
}

pub fn module() -> Arc<dyn domain::ConsoleModule> {
    Arc::new(ConsoleModule::builder()
        .with_component_parameters::<progress_notifier::ConsoleErrorNotifier>(progress_notifier::ConsoleErrorNotifierParameters {
            progress: MultiProgress::new(),
            progress_style: ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:100.cyan/blue} {pos:>11}/{len:11} {msg}")
                .progress_chars("##-"),
        })
        .build())
}
