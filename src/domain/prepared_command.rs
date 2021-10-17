use std::sync::Arc;

use crate::domain::model::Command;
use crate::domain::ports;

pub struct PreparedCommand {
    pub record_finder: Arc<dyn ports::RecordFinder>,
    pub progress_notifier: Arc<dyn ports::ProgressNotifier>,
    pub topics_finder: Arc<dyn ports::TopicsFinder>,

    pub(crate) cmd: Command,
}

impl PreparedCommand {
    pub(crate) fn execute(&self) {
        match &self.cmd {
            Command::QueryByKey(config, topics_matcher, criteria) => {
                self.topics_finder
                    .find_by(topics_matcher)
                    .flat_map(|topic| self.record_finder.find_by(topic, criteria))
                    .for_each(|rec| self.progress_notifier
                        .notify(serde_json::to_string(&rec).unwrap_or("Ups".to_string()).as_str()))
            }
            _ => { self.progress_notifier.notify("Command not found") }
        };
    }
}
