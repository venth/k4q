use std::sync::Arc;

use do_notation::m;
use rdkafka::ClientConfig;
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::StreamConsumer;

use crate::domain::model::{ApplicationProperties, ApplicationPropertiesExt, K4fqError};
use crate::domain::ports;
use crate::domain::ports::{KafkaSession, QueryRangeEstimator, RecordFinder, TopicsFinder};
use crate::kafka::properties::KafkaProperties;
use crate::kafka::query_range_estimator::KafkaQueryRangeEstimator;
use crate::kafka::record_finder::KafkaRecordFinder;
use crate::kafka::topics_finder::KafkaTopicsFinder;

#[derive(shaku::Component)]
#[shaku(interface = ports::KafkaSessionFactory)]
pub struct RdKafkaSessionFactory {}

pub struct RdKafkaSession {
    record_finder: Arc<dyn ports::RecordFinder>,
    topics_finder: Arc<dyn ports::TopicsFinder>,
    query_range_estimator: Arc<dyn ports::QueryRangeEstimator>,
}


impl ports::KafkaSession for RdKafkaSession {
    fn topics_finder(&self) -> Arc<dyn TopicsFinder> {
        self.topics_finder.clone()
    }

    fn query_range_estimator(&self) -> Arc<dyn QueryRangeEstimator> {
        self.query_range_estimator.clone()
    }

    fn record_finder(&self) -> Arc<dyn RecordFinder> {
        self.record_finder.clone()
    }
}

impl ports::KafkaSessionFactory for RdKafkaSessionFactory {
    fn create(&self, properties: Box<dyn ApplicationProperties>) -> Result<Box<dyn ports::KafkaSession>, K4fqError> {
        m! {
            config <- Self::read_kafka_configuration_from(properties);
            record_finder <- Self::create_record_finder(&config);
            topics_finder <- Self::create_topics_finder(&config);
            query_range_estimator <- Self::create_query_range_estimator(&config);

            return Box::new(RdKafkaSession {
                record_finder,
                topics_finder,
                query_range_estimator,
            }) as Box<dyn KafkaSession>;
        }
    }
}

impl RdKafkaSessionFactory {
    fn read_kafka_configuration_from(properties: Box<dyn ApplicationProperties>) -> Result<KafkaProperties, K4fqError> {
        properties
            .properties_by("kafka")
            .and_then(|props| props.as_ref().try_collect())
    }

    fn create_kafka_consumer(props: &KafkaProperties) -> Result<StreamConsumer, K4fqError> {
        ClientConfig::new()
            .set("group.id", props.group.id.to_string())
            .set("bootstrap.servers", props.bootstrap.servers.join(","))
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "true")
            //.set("statistics.interval.ms", "30000")
            //.set("auto.offset.reset", "smallest")
            .set_log_level(RDKafkaLogLevel::Debug)
            .create()
            .map_err(|e| K4fqError::KafkaError(e.to_string()))
    }

    fn create_record_finder(config: &KafkaProperties) -> Result<Arc<dyn RecordFinder>, K4fqError> {
        Self::create_kafka_consumer(config)
            .map(KafkaRecordFinder::new)
            .map(Arc::new)
            .map(|t| t as Arc<dyn RecordFinder>)
    }

    fn create_topics_finder(config: &KafkaProperties) -> Result<Arc<dyn TopicsFinder>, K4fqError> {
        Self::create_kafka_consumer(config)
            .map(KafkaTopicsFinder::new)
            .map(Arc::new)
            .map(|t| t as Arc<dyn TopicsFinder>)
    }

    fn create_query_range_estimator(config: &KafkaProperties) -> Result<Arc<dyn QueryRangeEstimator>, K4fqError> {
        Self::create_kafka_consumer(config)
            .map(KafkaQueryRangeEstimator::new)
            .map(Arc::new)
            .map(|t| t as Arc<dyn QueryRangeEstimator>)
    }
}