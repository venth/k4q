use crate::cli;
use crate::domain::action;
use crate::domain::action::Action;
use crate::domain::port::ActionRecognizer;
use crate::domain::record::Record;

pub fn new(action_recognizer: Box<dyn ActionRecognizer>) -> Box<App> {
    Box::new(App {
        action_recognizer: Box::new(action_recognizer)
    })
}

impl ActionRecognizer for App {
    fn recognize(&self, args: &Vec<&str>) -> Option<Box<dyn Action>> {
        self.action_recognizer.recognize(args)
    }
}

pub struct App {
    action_recognizer: Box<dyn ActionRecognizer>,
}

impl App {
    pub fn run(&self, args: &Vec<&str>) {
        self
            .recognize(args)
            .unwrap_or(action::empty(format!("Unsupported action for: {:?}", args)))
            .execute()
            .for_each(|rec| println!("Found record => {:?}", rec));
    }
}
