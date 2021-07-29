use std::sync::Arc;

use shaku::{HasComponent, module};

pub(crate) mod port;
pub(crate) mod action;
pub(crate) mod service;
mod app;


pub trait CliModule: HasComponent<dyn port::ActionRecognizer> {}

module! {
    pub DomainModule {
        components = [app::AppImpl],
        providers = [],

        use CliModule {
            components = [port::ActionRecognizer],
            providers = [],
        }
    }
}

pub fn domain_module(cli_module: Arc<dyn CliModule>) -> DomainModule {
    DomainModule::builder(cli_module)
        .build()
}