use std::sync::Arc;

use shaku::Component;

use crate::domain::action;
use crate::domain::port;
use crate::domain::service;

#[derive(Component)]
#[shaku(interface = service::App)]
pub struct AppImpl {
    #[shaku(inject)]
    action_recognizer: Arc<dyn port::ActionRecognizer>,
}

impl service::App for AppImpl {
    fn run<'a>(&self, args: &'a Vec<&'a str>) {
        self.action_recognizer
            .recognize(&args)
            .unwrap_or(Box::new(action::no_op()))
            .as_ref()
            .execute();
    }
}
