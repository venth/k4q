use crate::domain::action;
use crate::domain::port::ActionRecognizer;

pub fn new(action_recognizer: &impl ActionRecognizer) -> App {
    App {
        action_recognizer,
    }
}

pub struct App<'dep> {
    action_recognizer: &'dep dyn ActionRecognizer,
}

impl<'dep> App<'dep> {
    pub fn run<'a>(&self, args: &'a Vec<&'a str>) {
        self.action_recognizer
            .recognize(&args)
            .unwrap_or(Box::new(action::no_op()))
            .as_ref()
            .execute();
    }
}
