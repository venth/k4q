use crate::domain::criteria::Criteria;
use crate::domain::record::Record;

pub(crate) fn empty(message: String) -> Box<dyn Action> {
    Box::new(EmptyAction { message })
}

pub trait Action {
    fn execute(&self) -> Box<dyn Iterator<Item=Record>>;
}

pub trait ActionFactory {
    fn using(&self, criteria: Box<dyn Criteria>, topics: Vec<&str>) -> Box<dyn Action>;
}

impl Action for Box<dyn Action> {
    fn execute(&self) -> Box<dyn Iterator<Item=Record>> {
        self.as_ref().execute()
    }
}

impl Action for EmptyAction {
    fn execute(&self) -> Box<dyn Iterator<Item=Record>> {
        println!("{}", self.message);
        Box::new(std::iter::empty())
    }
}

impl Action for Box<EmptyAction> {
    fn execute(&self) -> Box<dyn Iterator<Item=Record>> {
        self.as_ref().execute()
    }
}

#[derive(Debug)]
struct EmptyAction {
    message: String,
}
