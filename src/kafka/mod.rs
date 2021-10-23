use std::sync::Arc;

use shaku::module;

use crate::domain;

mod record_finder;
mod topics_finder;
mod query_range_estimator;
mod stream_consumer_factory;

module! {
    pub KafkaModule: domain::KafkaModule {
        components = [
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

