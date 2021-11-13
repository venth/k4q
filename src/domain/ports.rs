use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;

use futures::Stream;
use shaku::Interface;

use crate::domain::model::{ApplicationProperties, Command, Count, EstimatedQueryRange, K4QError, QueryRange, Topic, TopicsMatcherType};
use crate::domain::model::Progress;
use crate::domain::model::Record;
use crate::domain::model::TopicName;

pub trait RecordFinder: Interface {
    fn find_by<'a>(&self, topic_name: &'a TopicName) -> Pin<Box<dyn Stream<Item=Record>>>;
}

pub trait CommandRecognizer: Interface {
    fn recognize(&self, args: &Vec<&str>) -> Option<Command>;
}

pub trait ProgressNotifier: Interface {
    fn notify(&self, message: &str);
    fn start(&self, estimated_max_size: &Count) -> Arc<dyn Progress>;
}

pub trait TopicsFinder: Interface {
    fn find_by<'a>(&self, topics_matcher_type: &'a TopicsMatcherType) -> Pin<Box<dyn Stream<Item=Topic> + 'a>>;
}

pub trait QueryRangeEstimator: Interface {
    fn estimate(&self, topic: &Topic, query_range: &QueryRange) -> EstimatedQueryRange;
}

pub trait ConfiguredContextFactory: Interface {
    fn create(&self, properties: Box<dyn ApplicationProperties>) -> Result<Box<dyn ConfiguredContext>, K4QError>;
}

pub trait ConfiguredContext {
    fn topics_finder(&self) -> Arc<dyn TopicsFinder>;
    fn query_range_estimator(&self) -> Arc<dyn QueryRangeEstimator>;
    fn record_finder(&self) -> Arc<dyn RecordFinder>;
}

pub trait PropertiesLocationProvider: Interface {
    fn provide(&self, args: &Vec<&str>) -> Option<PathBuf>;
}

pub trait PropertiesLoader: Interface {
    fn load(&self, config_location: &Path) -> Result<Box<dyn ApplicationProperties>, K4QError>;
}
