use std::sync::Arc;

use shaku::{HasComponent, module, ModuleInterface};

use crate::domain::service::ActionFactory;

pub(crate) mod port;
pub(crate) mod action;
pub(crate) mod service;
mod app;
pub(crate) mod criteria;
pub(crate) mod record;
pub(crate) mod query;
pub(crate) mod command;


pub trait CliModule: HasComponent<dyn port::ActionRecognizer> + HasComponent<dyn port::CommandRecognizer> {}

pub trait KafkaModule: HasComponent<dyn port::RecordFinder> {}

pub trait Module: HasComponent<dyn service::App> {}

module! {
    DomainModule: Module {
        components = [app::AppImpl],
        providers = [],

        use CliModule {
            components = [port::ActionRecognizer, port::CommandRecognizer],
            providers = [],
        },

        use KafkaModule {
            components = [port::RecordFinder],
            providers = [],
        },
    }
}

pub fn module(cli_module: Arc<dyn CliModule>, kafka_module: Arc<dyn KafkaModule>) -> Arc<dyn Module> {
    Arc::new(DomainModule::builder(cli_module, kafka_module)
        .build())
}