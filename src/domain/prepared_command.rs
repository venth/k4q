use std::sync::Arc;

use rxrust::observable;
use rxrust::observable::{Observable, SubscribeNext};
use rxrust::ops::box_it::{BoxObservable, LocalBoxOp};

use crate::domain::model::{Command, Criteria, Progress, Record, TopicName, TopicsMatcherType};
use crate::domain::ports;

pub struct PreparedCommand {
    pub record_finder: Arc<dyn ports::RecordFinder>,
    pub progress_notifier: Arc<dyn ports::ProgressNotifier>,
    pub topics_finder: Arc<dyn ports::TopicsFinder>,

    pub(crate) cmd: Command,
}

impl PreparedCommand {
    pub fn execute(&self) {
        match &self.cmd {
            Command::QueryByKey(
                config,
                topics_matcher,
                criteria) => self.execute_query_by_key(topics_matcher, criteria),
            _ => self.progress_notifier.notify("Command not found"),
        };
    }

    fn execute_query_by_key(&self, topics_matcher: &TopicsMatcherType, criteria: &Box<dyn Criteria>) {
        self.topics_finder
            .find_by(topics_matcher)
            .map(|topic_name| TopicQuery::new(topic_name, self.progress_notifier.start()))
            .flat_map(|query| self.record_finder
                .find_by(query.topic_name.clone())
                .map(move |record| query.resulted_with(record)))
            .subscribe(|result| result.unwrap().notify());
    }
}

struct TopicQuery {
    topic_name: TopicName,
    progress: Arc<dyn Progress>,
}

impl TopicQuery {
    fn new(topic_name: TopicName, progress: Arc<dyn Progress>) -> Box<TopicQuery> {
        Box::new(TopicQuery { topic_name, progress })
    }

    fn resulted_with(&self, record: Record) -> Result<QueryResult, ()> {
        Result::Ok(QueryResult {
            progress: self.progress.clone(),
            record,
        })
    }
}

struct QueryResult {
    progress: Arc<dyn Progress>,
    record: Record,
}

impl QueryResult {
    pub fn notify(self) {
        self.progress.message(serde_json::to_string(&self.record).unwrap_or("Ups".to_string()).as_str());
        self.progress.increase();
    }
}
