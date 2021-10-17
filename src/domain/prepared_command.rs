use std::sync::Arc;

use crate::domain::model::Command;
use crate::domain::ports;

pub struct PreparedCommand {
    pub(crate) record_finder: Arc<dyn ports::RecordFinder>,
    pub(crate) progress_notifier: Arc<dyn ports::ProgressNotifier>,

    pub(crate) cmd: Command,
}

impl PreparedCommand {
    pub(crate) fn execute(&self) {
        match &self.cmd {
            Command::QueryByKey(config, topics_matcher, criteria) => {
                let recs = self.record_finder
                    .find_by(vec! {"123"}, criteria);

                recs.for_each(|rec| self.progress_notifier
                    .notify(serde_json::to_string(&rec).unwrap_or("Ups".to_string()).as_str()))
            }
            _ => { self.progress_notifier.notify("Command not found") }
        };
    }
}
