use std::pin::Pin;
use std::sync::Arc;

use futures::{future, stream, Stream, StreamExt};

use crate::domain::model::{Command, Criteria, Progress, Record, Topic, TopicName, TopicsMatcherType};
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
        let t = self.topics_finder
            .find_by(topics_matcher)
            .map(|topic| self.initiate_query(topic))
            .map(|query| query.resulted_with(self.record_finder.find_by(query.topic_name())))
            .flat_map(|result| result.to_presentable())
            .for_each(|f| {
                f();
                future::ready(())
            });

        futures::executor::block_on(t);
    }

    fn initiate_query(&self, topic: Topic) -> TopicQuery {
        let record_count = topic.record_count();
        TopicQuery::new(topic, self.progress_notifier.start(&record_count))
    }
}

struct TopicQuery {
    topic: Topic,
    progress: Arc<dyn Progress>,
}

impl TopicQuery {
    fn new(topic: Topic, progress: Arc<dyn Progress>) -> TopicQuery {
        TopicQuery { topic, progress }
    }

    fn resulted_with(&self, result: Pin<Box<dyn Stream<Item=Record>>>) -> QueryResult {
        QueryResult {
            progress: self.progress.clone(),
            result,
        }
    }

    fn topic_name(&self) -> &TopicName {
        &self.topic.topic_name
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