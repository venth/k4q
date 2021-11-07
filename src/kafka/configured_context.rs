use std::pin::Pin;
use std::sync::Arc;

use futures::Stream;

use crate::domain::model::{ApplicationProperties, EstimatedQueryRange, QueryRange, Record, Topic, TopicName, TopicsMatcherType};
use crate::domain::ports;
use crate::domain::ports::{ConfiguredContext, QueryRangeEstimator, RecordFinder, TopicsFinder};

#[derive(shaku::Component)]
#[shaku(interface = ports::ConfiguredContextFactory)]
pub struct KafkaConfiguredContextFactory {
}

pub struct KafkaConfiguredContext {
    record_finder: Arc<dyn ports::RecordFinder>,
    topic_finder: Arc<dyn ports::TopicsFinder>,
    query_range_estimator: Arc<dyn ports::QueryRangeEstimator>,
}

impl QueryRangeEstimator for KafkaConfiguredContext {
    fn estimate(&self, topic: &Topic, query_range: &QueryRange) -> EstimatedQueryRange {
        self.query_range_estimator.estimate(topic, query_range)
    }
}

impl RecordFinder for KafkaConfiguredContext {
    fn find_by<'a>(&self, topic_name: &'a TopicName) -> Pin<Box<dyn Stream<Item=Record>>> {
        self.record_finder.find_by(topic_name)
    }
}

impl TopicsFinder for KafkaConfiguredContext {
    fn find_by<'a>(&self, topics_matcher_type: &'a TopicsMatcherType) -> Pin<Box<dyn Stream<Item=Topic> + 'a>> {
        self.topic_finder.find_by(topics_matcher_type)
    }
}

impl ports::ConfiguredContextFactory for KafkaConfiguredContextFactory {
    fn create(&self, properties: &dyn ApplicationProperties) -> Box<dyn ConfiguredContext> {
        todo!()
    }
}