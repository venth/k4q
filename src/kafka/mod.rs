use std::sync::Arc;

use shaku::module;

use crate::domain;

mod record_finder;

module! {
    pub KafkaModule: domain::KafkaModule {
        components = [
            record_finder::KafkaRecordFinder,
        ],
        providers = [],
    }
}

pub fn module() -> Arc<dyn domain::KafkaModule> {
    Arc::new(KafkaModule::builder()
        .build())
}
