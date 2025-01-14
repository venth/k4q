use std::iter;
use std::sync::Arc;

use rayon::iter::ParallelIterator;
use rayon::prelude::ParallelBridge;

use crate::domain::model::{Command, Criteria, EstimatedQueryRange, K4fqError, Progress, QueryRange, Record, TopicName, TopicsMatcherType};
use crate::domain::ports;

pub struct PreparedCommand {
    pub configured_context: Arc<dyn ports::KafkaSession>,
    pub progress_notifier: Arc<dyn ports::ProgressNotifier>,

    pub cmd: Command,
}


impl PreparedCommand {
    pub fn execute(&self) {
        match &self.cmd {
            Command::QueryByKey(
                topics_matcher,
                criteria) => {
                self.execute_query_by_key(topics_matcher, criteria)
            }
            _ => self.progress_notifier.notify("Command not found"),
        };
    }


    fn execute_query_by_key(&self, topics_matcher: &TopicsMatcherType, _: &Box<dyn Criteria>) {
        let topics_finder = self.configured_context.topics_finder();
        let query_range_estimator = self.configured_context.query_range_estimator();
        let record_finder = self.configured_context.record_finder();
        topics_finder.find_by(topics_matcher)
            .par_bridge()
            .map(|r| r.map(|topic| query_range_estimator.estimate(&topic, &QueryRange::Whole)))
            .map(|r| r.map(|query_range| self.initiate_query(query_range)))
            .map(|r| r.map(|query| query.resulted_with(record_finder.find_by(query.topic_name()))))
            .map(|r| r.map(|q| q.to_presentable()))
            .flat_map(|q| q
                .unwrap_or_else(|err|
                    Box::new(iter::once(notify_about_error(err))))
                .par_bridge())
            .for_each(|n| {
                n.notify()
            })

        // .try_for_each_concurrent(10, |n: QueryResult| async {
        //     n.print_record()
        //     Ok(println!("{:?}", n))
        // });
        // .flat_map(|result| r.to_presentable())
        // .for_each(|f| {
        //     f();
        //     future::ready(())
        // });

        // futures::executor::block_on(t);
    }

    fn initiate_query(&self, estimated_range: EstimatedQueryRange) -> TopicQuery {
        let record_count = estimated_range.estimated_record_count;
        TopicQuery::new(estimated_range.topic_name, self.progress_notifier.start(&record_count))
    }
}


fn notify_about_error<'a>(err: K4fqError) -> Box<dyn Notification + Sync + Send + 'a> {
    Box::new(ErrorNotification::new(err))
}


struct ErrorNotification {
    err: K4fqError,
}

impl ErrorNotification {
    fn new(err: K4fqError) -> Self {
        Self { err }
    }
}

unsafe impl Send for ErrorNotification {}

impl Notification for ErrorNotification {
    fn notify(&self) {
        eprintln!("ERROR -> {:?}", self.err);
    }
}

struct TopicQuery {
    topic_name: TopicName,
    progress: Arc<dyn Progress + Sync + Send>,
}

unsafe impl Send for TopicQuery {}

unsafe impl Sync for TopicQuery {}

impl TopicQuery {
    fn new(topic_name: TopicName, progress: Arc<dyn Progress + Sync + Send>) -> TopicQuery {
        TopicQuery { topic_name, progress }
    }

    fn resulted_with(&self, result: Box<dyn Iterator<Item=Record> + Send + Sync>) -> QueryResult {
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
    progress: Arc<dyn Progress + Sync + Send>,
    result: Box<dyn Iterator<Item=Record> + Sync + Send>,
}

unsafe impl Send for QueryResult {}

unsafe impl Sync for QueryResult {}

impl QueryResult {
    fn to_presentable<'a>(self) -> Box<dyn Iterator<Item=Box<dyn Notification + Sync + Send>> + Sync + Send + 'a> {
        let QueryResult { progress, result } = self;
        let progress_result = progress.clone();
        let progress_finish = progress.clone();

        Box::new(result
            .map(move |rec| QueryResult::notify(progress_result.clone(), rec))
            .chain(iter::once(QueryResult::finish(progress_finish))))
    }

    fn notify<'a>(progress: Arc<dyn Progress + Sync + Send>, rec: Record) -> Box<dyn Notification + Sync + Send + 'a> {
        Box::new(FoundResultNotification::new(progress, rec))
    }

    fn finish<'a>(progress: Arc<dyn Progress + Sync + Send>) -> Box<dyn Notification + Sync + Send + 'a> {
        Box::new(FinishingNotification::new(progress))
    }
}

trait Notification: Sync + Send {
    fn notify(&self);
}

impl Notification for Box<dyn Notification> {
    fn notify(&self) {
        self.as_ref().notify();
    }
}

struct FinishingNotification {
    progress: Arc<dyn Progress + Sync + Send>,
}

impl FinishingNotification {
    fn new(progress: Arc<dyn Progress + Sync + Send>) -> Self {
        Self { progress }
    }
}

unsafe impl Send for FinishingNotification {}

unsafe impl Sync for FinishingNotification {}

impl Notification for FinishingNotification {
    fn notify(&self) {
        self.progress.finish();
    }
}

struct FoundResultNotification {
    progress: Arc<dyn Progress + Sync + Send>,
    record: Record,
}

impl FoundResultNotification {
    pub fn new(progress: Arc<dyn Progress + Sync + Send>, record: Record) -> Self {
        Self { progress, record }
    }
}

unsafe impl Send for FoundResultNotification {}

unsafe impl Sync for FoundResultNotification {}

impl Notification for FoundResultNotification {
    fn notify(&self) {
        self.progress.message(serde_json::to_string(&self.record).unwrap_or("Ups".to_string()).as_str());
        self.progress.increase();
    }
}