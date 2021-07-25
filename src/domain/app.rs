use crate::domain::action;
use crate::domain::port::ActionRecognizer;

pub fn new<'dep, 'a>(action_recognizer: &'dep impl ActionRecognizer<'a>) -> App<'dep, 'a> {
    App {
        action_recognizer,
    }
}

pub struct App<'dep, 'a> {
    action_recognizer: &'dep dyn ActionRecognizer<'a>,
}

impl<'dep, 'a> App<'dep, 'a> {
    pub fn run<'b>(&self, args: &'b Vec<&'b str>) {
        self.action_recognizer
            .recognize(&args)
            .unwrap_or(Box::new(action::no_op()))
            .as_ref()
            .execute();
    }
}
