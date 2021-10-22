use std::sync::Arc;

use shaku::module;

use crate::domain;

mod record_finder;
mod topics_finder;
mod kafka_query_range_estimator;

module! {
    pub KafkaModule: domain::KafkaModule {
        components = [
            record_finder::KafkaRecordFinder,
            topics_finder::KafkaTopicsFinder,
            kafka_query_range_estimator::KafkaQueryRangeEstimator,
        ],
        providers = [],
    }
}

pub fn module() -> Arc<dyn domain::KafkaModule> {
    Arc::new(KafkaModule::builder()
        .build())
}
