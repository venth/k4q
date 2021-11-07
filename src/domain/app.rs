use std::sync::Arc;

use home::home_dir;
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
    configured_context_factory: Arc<dyn ports::ConfiguredContextFactory>,

    #[shaku(inject)]
    progress_notifier: Arc<dyn ports::ProgressNotifier>,

    #[shaku(inject)]
    properties_source: Arc<dyn ports::PropertiesSource>,
}

impl service::App for AppImpl {
    fn run<'a>(&self, args: &'a Vec<&'a str>) {
        self.command_of(&args)
            .execute();
    }
}

impl AppImpl {
    fn command_of(&self, args: &&Vec<&str>) -> PreparedCommand {
        let kafka_config = home_dir()
            .map(|p| p.join(".k4q/config.yaml"))
            .expect("cannot find config file - ups");

        let props = self.properties_source.load(&kafka_config)
            .expect("cannot load props");
        let  configured_context: Arc<dyn ports::ConfiguredContext> = self.configured_context_factory.clone()
            .create(props.as_ref())
            .into();
        self.command_recognizer
            .recognize(&args)
            .map(|cmd| PreparedCommand {
                configured_context: configured_context.clone(),
                progress_notifier: self.progress_notifier.clone(),
                cmd,
            })
            .unwrap_or_else(|| PreparedCommand {
                configured_context: configured_context.clone(),
                progress_notifier: self.progress_notifier.clone(),
                cmd: Command::CommandNotRecognized,
            })
    }
}