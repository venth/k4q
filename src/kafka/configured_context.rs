use std::sync::Arc;

use rdkafka::ClientConfig;
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::StreamConsumer;

use crate::domain::model::{ApplicationProperties, ApplicationPropertiesExt};
use crate::domain::ports;
use crate::domain::ports::{QueryRangeEstimator, RecordFinder, TopicsFinder};
use crate::kafka::properties::KafkaProperties;

#[derive(shaku::Component)]
#[shaku(interface = ports::ConfiguredContextFactory)]
pub struct KafkaConfiguredContextFactory {
    #[shaku(inject)]
    record_finder: Arc<dyn ports::RecordFinder>,
    #[shaku(inject)]
    topic_finder: Arc<dyn ports::TopicsFinder>,
    #[shaku(inject)]
    query_range_estimator: Arc<dyn ports::QueryRangeEstimator>,
}

pub struct KafkaConfiguredContext {
    record_finder: Arc<dyn ports::RecordFinder>,
    topic_finder: Arc<dyn ports::TopicsFinder>,
    query_range_estimator: Arc<dyn ports::QueryRangeEstimator>,
}


impl ports::ConfiguredContext for KafkaConfiguredContext {
    fn topics_finder(&self) -> Arc<dyn TopicsFinder> {
        self.topic_finder.clone()
    }

    fn query_range_estimator(&self) -> Arc<dyn QueryRangeEstimator> {
        self.query_range_estimator.clone()
    }

    fn record_finder(&self) -> Arc<dyn RecordFinder> {
        self.record_finder.clone()
    }
}

impl ports::ConfiguredContextFactory for KafkaConfiguredContextFactory {
    fn create(&self, properties: Box<dyn ApplicationProperties>) -> Box<dyn ports::ConfiguredContext> {
        let config: KafkaProperties = properties
            .properties_by("kafka")
            .expect("something wrong")
            .as_ref()
            .try_collect()
            .expect("wrong, wrong");

        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", config.group.id)
            .set("bootstrap.servers", config.bootstrap.servers.join(","))
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "true")
            //.set("statistics.interval.ms", "30000")
            //.set("auto.offset.reset", "smallest")
            .set_log_level(RDKafkaLogLevel::Debug)
            .create()
            .expect("Consumer creation failed");

        Box::new(KafkaConfiguredContext {
            record_finder: self.record_finder.clone(),
            topic_finder: self.topic_finder.clone(),
            query_range_estimator: self.query_range_estimator.clone(),
        })
    }
}