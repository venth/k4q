use std::path::{Path, PathBuf};
use std::sync::Arc;

use shaku::Interface;

use crate::domain::model::Progress;
use crate::domain::model::Record;
use crate::domain::model::TopicName;
use crate::domain::model::{ApplicationProperties, Command, Count, EstimatedQueryRange, K4fqError, QueryRange, Topic, TopicsMatcherType};

pub trait RecordFinder: Interface {
    fn find_by<'a>(&self, topic_name: &'a TopicName) -> Box<dyn Iterator<Item=Record> + Sync + Send>;
}

pub trait CommandRecognizer: Interface {
    fn recognize(&self, args: &Vec<&str>) -> Option<Command>;
}

pub trait ProgressNotifier: Interface {
    fn notify(&self, message: &str);
    fn start(&self, estimated_max_size: &Count) -> Arc<dyn Progress + Sync + Send>;
}

pub trait TopicsFinder: Interface {
    fn find_by<'a>(&'a self, topics_matcher_type: &'a TopicsMatcherType) -> Box<dyn Iterator<Item=Result<Topic, K4fqError>> + Send + 'a>;
}

pub trait QueryRangeEstimator: Interface {
    fn estimate(&self, topic: &Topic, query_range: &QueryRange) -> EstimatedQueryRange;
}

pub trait KafkaSessionFactory: Interface {
    fn create(&self, properties: Box<dyn ApplicationProperties>) -> Result<Box<dyn KafkaSession>, K4fqError>;
}

pub trait KafkaSession: Sync + Send {
    fn topics_finder(&self) -> Arc<dyn TopicsFinder>;
    fn query_range_estimator(&self) -> Arc<dyn QueryRangeEstimator>;
    fn record_finder(&self) -> Arc<dyn RecordFinder>;
}

pub trait PropertiesLocationProvider: Interface {
    fn provide(&self, args: &Vec<&str>) -> Option<PathBuf>;
}

pub trait PropertiesLoader: Interface {
    fn load(&self, config_location: &Path) -> Result<Box<dyn ApplicationProperties>, K4fqError>;
}
