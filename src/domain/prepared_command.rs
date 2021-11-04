use std::pin::Pin;
use std::sync::Arc;

use futures::{future, stream, Stream, StreamExt};

use crate::domain::model::{CollectableProperties, Command, Criteria, EstimatedQueryRange, Progress, QueryRange, Record, TopicName, TopicsMatcherType};
use crate::domain::ports;

pub struct PreparedCommand {
    pub record_finder: Arc<dyn ports::RecordFinder>,
    pub progress_notifier: Arc<dyn ports::ProgressNotifier>,
    pub topics_finder: Arc<dyn ports::TopicsFinder>,
    pub query_range_estimator: Arc<dyn ports::QueryRangeEstimator>,
    pub properties_source: Arc<dyn ports::PropertiesSource>,

    pub cmd: Command,
}


#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Kafka {}


impl PreparedCommand {
    pub fn execute(&self) {
        match &self.cmd {
            Command::QueryByKey(
                config,
                topics_matcher,
                criteria) => {
                let properties = self.properties_source.load(
                    config.as_ref().as_ref().unwrap().location.as_path());

                let kafka: Kafka = properties
                    .expect("properties are present")
                    .properties_by("kafka")
                    .expect("kafka is defined")
                    .as_ref()
                    .try_collect()
                    .expect("structure is wrong");

                println!("Kafka :{}", serde_json::to_string(&kafka).unwrap_or("Ups".to_string()).as_str() );
                self.execute_query_by_key(topics_matcher, criteria)
            }
            _ => self.progress_notifier.notify("Command not found"),
        };
    }


    fn execute_query_by_key(&self, topics_matcher: &TopicsMatcherType, criteria: &Box<dyn Criteria>) {
        let t = self.topics_finder
            .find_by(topics_matcher)
            .map(|topic| self.query_range_estimator.estimate(&topic, &QueryRange::Whole))
            .map(|query_range| self.initiate_query(query_range))
            .map(|query| query.resulted_with(self.record_finder.find_by(query.topic_name())))
            .flat_map(|result| result.to_presentable())
            .for_each(|f| {
                f();
                future::ready(())
            });

        futures::executor::block_on(t);
    }

    fn initiate_query(&self, estimated_range: EstimatedQueryRange) -> TopicQuery {
        let record_count = estimated_range.estimated_record_count;
        TopicQuery::new(estimated_range.topic_name, self.progress_notifier.start(&record_count))
    }
}

struct TopicQuery {
    topic_name: TopicName,
    progress: Arc<dyn Progress>,
}

impl TopicQuery {
    fn new(topic_name: TopicName, progress: Arc<dyn Progress>) -> TopicQuery {
        TopicQuery { topic_name, progress }
    }

    fn resulted_with(&self, result: Pin<Box<dyn Stream<Item=Record>>>) -> QueryResult {
        QueryResult {
            progress: self.progress.clone(),
            result,
        }
    }

    fn topic_name(&self) -> &TopicName {
        &self.topic_name
    }
}


struct QueryResult {
    progress: Arc<dyn Progress>,
    result: Pin<Box<dyn Stream<Item=Record>>>,
}

impl QueryResult {
    fn to_presentable<'a>(self) -> Pin<Box<dyn Stream<Item=Box<dyn FnOnce() + 'a>> + 'a>> {
        let QueryResult { progress, result } = self;
        let progress_result = progress.clone();
        let progress_finish = progress.clone();

        Box::pin(result
            .map(move |rec| QueryResult::notify(progress_result.clone(), rec))
            .chain(stream::once(async { QueryResult::finish(progress_finish) })))
    }

    fn notify<'a>(progress: Arc<dyn Progress>, rec: Record) -> Box<dyn FnOnce() + 'a> {
        Box::new(move || {
            progress.message(serde_json::to_string(&rec).unwrap_or("Ups".to_string()).as_str());
            progress.increase();
        })
    }

    fn finish<'a>(progress: Arc<dyn Progress>) -> Box<dyn FnOnce() + 'a> {
        Box::new(move || progress.finish())
    }
}