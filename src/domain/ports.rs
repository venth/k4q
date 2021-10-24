use std::fs::File;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;

use futures::Stream;
use shaku::Interface;

use crate::domain::model::{Command, Count, EstimatedQueryRange, K4QError, Properties, QueryRange, Topic, TopicsMatcherType};
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
    fn create(&self, properties: &Properties) -> Box<dyn ConfiguredContext>;
}

pub trait ConfiguredContext: TopicsFinder + QueryRangeEstimator + RecordFinder {
}

pub trait PropertiesSource: Interface {
    fn load(&self, config_location: &Path) -> Result<Properties, K4QError>;
}
