use crate::domain::action::Action;
use crate::domain::port::ActionRecognizer;

pub fn new() -> impl ActionRecognizer {
    CliActionRecognizer {}
}

struct CliActionRecognizer {
}

impl ActionRecognizer for CliActionRecognizer {
    fn recognize<'a>(&self, args: &'a Vec<&'a str>) -> Option<Box<dyn Action +'a>> {
        Some(Box::new(DummyAction { args }))
    }
}

struct DummyAction<'a> {
    args: &'a Vec<&'a str>,
}

impl<'a> Action for DummyAction<'a> {
    fn execute(&self) {
        todo!()
    }
}