pub trait Action<'a> {
    fn execute(&self);
}

pub fn no_op<'a>() -> impl Action<'a> {
    NoOpAction {}
}

struct NoOpAction {

}

impl<'a> Action<'a> for NoOpAction {
    fn execute(&self) {}
}
