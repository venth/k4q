use crate::domain::action::Action;
use crate::domain::criteria::Criteria;
use crate::domain::record::Record;

pub trait RecordFinder {
    fn find_by<'a>(&self, topics: &'a Vec<String>,
               criteria: &'a dyn Criteria) -> &'a dyn Iterator<Item=Record>;
}

pub trait ActionRecognizer {
    fn recognize(&self, args: &Vec<&str>) -> Option<Box<dyn Action>>;
}

impl ActionRecognizer for Box<dyn ActionRecognizer> {
    fn recognize(&self, args: &Vec<&str>) -> Option<Box<dyn Action>> {
        self.as_ref().recognize(args)
    }
}
