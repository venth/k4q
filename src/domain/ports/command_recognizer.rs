use crate::domain::model;

pub(crate) trait CommandRecognizer {
    fn recognize(&self, args: &Vec<String>) -> model::RecognizedCommand;
}
