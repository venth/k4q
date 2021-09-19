use std::sync::Arc;

use shaku::Component;

use crate::domain::ports;
use crate::domain::service;

#[derive(Component)]
#[shaku(interface = service::App)]
pub struct AppImpl {
    #[shaku(inject)]
    command_recognizer: Arc<dyn ports::CommandRecognizer>,
}

impl service::App for AppImpl {
    fn run<'a>(&self, args: &'a Vec<&'a str>) {
        self.command_recognizer
            .recognize(&args);


/*        self.action_recognizer
            .recognize(&args)
            .unwrap_or(Box::new(action::no_op()))
            .as_ref()
            .execute();
*/    }
}
