#[cfg_attr(test, derive(Clone))]
pub(crate) enum RecognizedCommand {
    UnrecognizedCommand(Vec<String>),
}
