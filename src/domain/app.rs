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
    configured_context_factory: Arc<dyn ports::ConfiguredContextFactory>,

    #[shaku(inject)]
    progress_notifier: Arc<dyn ports::ProgressNotifier>,

    #[shaku(inject)]
    properties_location_provider: Arc<dyn ports::PropertiesLocationProvider>,

    #[shaku(inject)]
    properties_loader: Arc<dyn ports::PropertiesLoader>,
}

impl service::App for AppImpl {
    fn run<'a>(&self, args: &'a Vec<&'a str>) {
        self.command_of(&args)
            .execute();
    }
}

impl AppImpl {
    fn command_of(&self, args: &&Vec<&str>) -> PreparedCommand {
        let kafka_config = self.properties_location_provider
            .as_ref()
            .provide(args)
            .expect("cannot determine properties file location");

        let props = self.properties_loader.load(&kafka_config)
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