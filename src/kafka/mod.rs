use std::sync::Arc;

use shaku::module;

use crate::domain;

mod record_finder;
mod topics_finder;
mod query_range_estimator;
mod stream_consumer_factory;
mod properties;
mod configured_context;

module! {
    pub KafkaModule: domain::KafkaModule {
        components = [
            configured_context::KafkaConfiguredContextFactory,
            record_finder::KafkaRecordFinder,
            topics_finder::KafkaTopicsFinder,
            query_range_estimator::KafkaQueryRangeEstimator,
            stream_consumer_factory::KafkaStreamConsumerFactory,
        ],
        providers = [],
    }
}

pub fn module() -> Arc<dyn domain::KafkaModule> {
    Arc::new(KafkaModule::builder()
        .build())
}

