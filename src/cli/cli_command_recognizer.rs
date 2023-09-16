use std::sync::Arc;

use crate::cli::cli_parser::{ClapCliParserFactory, CliParserFactory};
use crate::domain::model as domainModel;
use crate::domain::ports;

struct CliCommandRecognizer {
    cli_parser: Arc<dyn CliParserFactory>,
}


pub(crate) fn new() -> impl ports::CommandRecognizer {
    return CliCommandRecognizer { cli_parser: Arc::new(ClapCliParserFactory {}) };
}

impl ports::CommandRecognizer for CliCommandRecognizer {
    fn recognize(&self, _: &Vec<String>) -> domainModel::RecognizedCommand {
        todo!()
    }
}
