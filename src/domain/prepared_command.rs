use std::sync::Arc;

use crate::domain::model::{Command, Criteria, Progress, Record, TopicsMatcherType};
use crate::domain::model::TopicName;
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
            .map(|query| query.resulted_with(self.record_finder.find_by(&query.topic_name)))
            .flat_map(|result| result.to_presentable())
            .for_each(|f| f())
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

    fn resulted_with(&self, result: Box<dyn Iterator<Item=Record>>) -> QueryResult {
        QueryResult {
            progress: self.progress.clone(),
            result,
        }
    }
}

struct QueryResult {
    progress: Arc<dyn Progress>,
    result: Box<dyn Iterator<Item=Record>>,
}

impl QueryResult {
    fn to_presentable<'a>(self) -> Box<dyn Iterator<Item=Box<dyn FnOnce() + 'a>> + 'a> {
        let QueryResult { progress, result } = self;
        let progress_result = progress.clone();
        let progress_finish = progress.clone();

        Box::new(
            result
                .map(move |rec| QueryResult::notify(progress_result.clone(), rec))
                .chain(std::iter::once(QueryResult::finish(progress_finish))))
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