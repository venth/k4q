use std::sync::Arc;

use crate::domain::model::ApplicationProperties;
use crate::domain::ports;
use crate::domain::ports::{QueryRangeEstimator, RecordFinder, TopicsFinder};

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
    fn create(&self, _properties: &dyn ApplicationProperties) -> Box<dyn ports::ConfiguredContext> {
        Box::new(KafkaConfiguredContext {
            record_finder: self.record_finder.clone(),
            topic_finder: self.topic_finder.clone(),
            query_range_estimator: self.query_range_estimator.clone(),
        })
    }
}