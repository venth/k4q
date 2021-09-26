use std::sync::Arc;

use shaku::Component;

use crate::domain::model::Command;
use crate::domain::ports;
use crate::domain::service;

#[derive(Component)]
#[shaku(interface = service::App)]
pub struct AppImpl {
    #[shaku(inject)]
    command_recognizer: Arc<dyn ports::CommandRecognizer>,

    #[shaku(inject)]
    progress_notifier: Arc<dyn ports::ProgressNotifier>,

    #[shaku(inject)]
    record_finder: Arc<dyn ports::RecordFinder>,
}

impl service::App for AppImpl {
    fn run<'a>(&self, args: &'a Vec<&'a str>) {
        let cmd = self.command_of(&args);
        self.execute(cmd)
    }
}

impl AppImpl {
    fn execute(&self, cmd: Command) {
        match cmd {
            Command::QueryByKey(config, topicsMatcher, criteria) => {
                let recs = self.record_finder
                    .find_by(vec!{"123"}, &criteria);

                recs.for_each(|rec| self.progress_notifier
                    .notify(serde_json::to_string(&rec).unwrap_or("Ups".to_string()).as_str()))
            }
            Command::CommandNotRecognized => { self.progress_notifier.notify("Command not found") }
        }
    }

    fn command_of(&self, args: &&Vec<&str>) -> Command {
        self.command_recognizer
            .recognize(&args)
            .unwrap_or_else(|| Command::CommandNotRecognized)
    }
}