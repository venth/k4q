use crate::domain::model;

pub(crate) trait CommandRecognizer: Send + Sync {
    fn recognize(&self, args: &Vec<String>) -> model::RecognizedCommand;
}
