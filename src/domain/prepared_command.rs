use std::iter;
use std::sync::Arc;

use itertools::Itertools;
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


    fn execute_query_by_key(&self, topics_matcher: &TopicsMatcherType, criteria: &Box<dyn Criteria>) {
        let topics_finder = self.configured_context.topics_finder();
        let query_range_estimator = self.configured_context.query_range_estimator();
        let record_finder = self.configured_context.record_finder();
        topics_finder.find_by(topics_matcher)
            .map_ok(|topic| query_range_estimator.estimate(&topic, &QueryRange::Whole))
            .map_ok(|query_range| self.initiate_query(query_range))
            .map_ok(|query| query.resulted_with(record_finder.find_by(query.topic_name())))
            .map_ok(|q| q.to_presentable())
            .flat_map(|q| q.unwrap_or_else(|err|
                Box::new(iter::once(notify_about_error(err)))))
            .for_each(|n| {
                n()
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

fn notify_about_error<'a>(err: K4fqError) -> Box<dyn FnOnce() + 'a> {
    Box::new(move || {
        eprintln!("ERROR -> {err:?}")
    })
}

struct TopicQuery {
    topic_name: TopicName,
    progress: Arc<dyn Progress>,
}

impl TopicQuery {
    fn new(topic_name: TopicName, progress: Arc<dyn Progress>) -> TopicQuery {
        TopicQuery { topic_name, progress }
    }

    fn resulted_with(&self, result: Box<dyn Iterator<Item=Record>>) -> QueryResult {
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
    result: Box<dyn Iterator<Item=Record>>,
}

impl QueryResult {
    fn to_presentable<'a>(self) -> Box<dyn Iterator<Item=Box<dyn FnOnce()>> + 'a> {
        let QueryResult { progress, result } = self;
        let progress_result = progress.clone();
        let progress_finish = progress.clone();

        Box::new(result
            .map(move |rec| QueryResult::notify(progress_result.clone(), rec))
            .chain(iter::once(QueryResult::finish(progress_finish))))
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