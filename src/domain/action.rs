use shaku::Interface;

use crate::domain::criteria::Criteria;
use crate::domain::record::Record;

pub trait Action {
    fn execute(&self);
}

impl Action for Box<dyn Action> {
    fn execute(&self) {
        self.as_ref().execute()
    }
}


pub fn no_op() -> impl Action {
    NoOpAction {}
}

#[derive(Debug)]
struct NoOpAction {

}

impl Action for NoOpAction {
    fn execute(&self) {}
}
