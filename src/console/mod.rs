use std::sync::Arc;

use shaku::module;

use crate::domain;

mod error_notifier;

module! {
    pub ConsoleModule: domain::ConsoleModule {
        components = [
            error_notifier::ConsoleErrorNotifier,
        ],
        providers = [],
    }
}

pub fn module() -> Arc<dyn domain::ConsoleModule> {
    Arc::new(ConsoleModule::builder()
        .build())
}
