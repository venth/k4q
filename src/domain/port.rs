use shaku::Interface;

use crate::domain::action::Action;

pub trait RecordFinder<'a> {}

pub trait ActionRecognizer: Interface {
    fn recognize<'a>(&self, args: &'a Vec<&'a str>) -> Option<Box<dyn Action + 'a>>;
}
