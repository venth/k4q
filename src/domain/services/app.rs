use crate::domain::ports;

pub(crate) fn new_app_runner(_: &impl ports::CommandRecognizer) -> impl Fn() {
    return || {};
}
