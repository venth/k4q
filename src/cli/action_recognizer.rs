use crate::domain::action;
use crate::domain::action::Action;
use crate::domain::port::ActionRecognizer;

pub fn new<'a>() -> impl ActionRecognizer<'a>{
    CliActionRecognizer {}
}

struct CliActionRecognizer {
}

impl<'a> ActionRecognizer<'a> for CliActionRecognizer {
    fn recognize<'b>(&self, args: &'b Vec<&'b str>) -> Option<Box<dyn Action<'a> +'a>> {
        Some(Box::new(action::no_op()))
    }
}