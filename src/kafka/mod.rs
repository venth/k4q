use std::sync::Arc;

use shaku::module;

use crate::domain;

mod record_finder;
mod topics_finder;
mod query_range_estimator;
mod stream_consumer_factory;
mod properties;
mod kafka_session;
mod partition_reader;
mod kafka_reader;
mod timeout_aware_stream_consumer;
mod metadata_reader;
mod partitions_reader;

module! {
    pub KafkaModule: domain::KafkaModule {
        components = [
            kafka_session::RdKafkaSessionFactory,
            stream_consumer_factory::KafkaStreamConsumerFactory,
        ],
        providers = [],
    }
}

pub fn module() -> Arc<dyn domain::KafkaModule> {
    Arc::new(KafkaModule::builder()
        .build())
}

