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
    error_notifier: Arc<dyn ports::ErrorNotifier>,
}

impl service::App for AppImpl {
    fn run<'a>(&self, args: &'a Vec<&'a str>) {
        let cmd = self.command_recognizer
            .recognize(&args)
            .unwrap_or_else(||Command::CommandNotRecognized);

        self.execute(cmd)

        /*        self.action_recognizer
                    .recognize(&args)
                    .unwrap_or(Box::new(action::no_op()))
                    .as_ref()
                    .execute();
        */
    }
}

impl AppImpl {
    fn execute(&self, cmd: Command) {
        match cmd {
            Command::QueryByKey(config, topicsMatcher, criteria) => {}
            Command::CommandNotRecognized => { self.error_notifier.notify("Command not found") }
        }
    }
}