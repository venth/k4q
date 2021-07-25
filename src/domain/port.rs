use crate::domain::action::Action;

pub trait RecordFinder<'a> {}

pub trait ActionRecognizer {
    fn recognize<'a>(&self, args: &'a Vec<&'a str>) -> Option<Box<dyn Action + 'a>>;
}
