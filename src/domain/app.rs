use std::sync::Arc;

use shaku::Component;

use crate::domain::model::Command;
use crate::domain::ports;
use crate::domain::prepared_command::PreparedCommand;
use crate::domain::service;

#[derive(Component)]
#[shaku(interface = service::App)]
pub struct AppImpl {
    #[shaku(inject)]
    command_recognizer: Arc<dyn ports::CommandRecognizer>,

    #[shaku(inject)]
    record_finder: Arc<dyn ports::RecordFinder>,

    #[shaku(inject)]
    progress_notifier: Arc<dyn ports::ProgressNotifier>,
}

impl service::App for AppImpl {
    fn run<'a>(&self, args: &'a Vec<&'a str>) {
        self.command_of(&args)
            .execute();
    }
}

impl AppImpl {
    fn command_of(&self, args: &&Vec<&str>) -> PreparedCommand {
        self.command_recognizer
            .recognize(&args)
            .map(|cmd| PreparedCommand {
                record_finder: self.record_finder.clone(),
                progress_notifier: self.progress_notifier.clone(),
                cmd,
            })
            .unwrap_or_else(|| PreparedCommand {
                record_finder: self.record_finder.clone(),
                progress_notifier: self.progress_notifier.clone(),
                cmd: Command::CommandNotRecognized,
            })
    }
}