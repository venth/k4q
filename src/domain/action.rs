pub trait Action {
    fn execute(&self);
}

pub fn no_op() -> impl Action {
    NoOpAction {}
}

struct NoOpAction {

}

impl Action for NoOpAction {
    fn execute(&self) {}
}
