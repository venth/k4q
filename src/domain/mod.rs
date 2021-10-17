use std::sync::Arc;

use shaku::{HasComponent, module};

pub(crate) mod ports;
pub(crate) mod action;
pub(crate) mod service;
mod app;
pub(crate) mod query;
pub(crate) mod model;
mod prepared_command;


pub trait CliModule: HasComponent<dyn ports::CommandRecognizer> {}

pub trait KafkaModule: HasComponent<dyn ports::RecordFinder> {}

pub trait ConsoleModule: HasComponent<dyn ports::ProgressNotifier> {}

pub trait Module: HasComponent<dyn service::App> {}

module! {
    DomainModule: Module {
        components = [app::AppImpl],
        providers = [],

        use CliModule {
            components = [ports::CommandRecognizer],
            providers = [],
        },

        use KafkaModule {
            components = [ports::RecordFinder],
            providers = [],
        },

        use ConsoleModule {
            components = [ports::ProgressNotifier],
            providers = []
        },
    }
}

pub fn module(cli_module: Arc<dyn CliModule>,
              kafka_module: Arc<dyn KafkaModule>,
              console_module: Arc<dyn ConsoleModule>) -> Arc<dyn Module> {
    Arc::new(DomainModule::builder(cli_module, kafka_module, console_module)
        .build())
}
