use std::sync::Arc;

use shaku;
use shaku::Component;

use crate::cli::cli_parser::CliParserFactory;
use crate::cli::command_selector::CommandSelector;
use crate::domain::model::Command;
use crate::domain::ports;

#[derive(Component)]
#[shaku(interface = ports::CommandRecognizer)]
pub struct CliCommandRecognizer {
    #[shaku(inject)]
    cli_parser: Arc<dyn CliParserFactory>,

    #[shaku(inject)]
    command_selector: Arc<dyn CommandSelector>,
}

impl ports::CommandRecognizer for CliCommandRecognizer {
    fn recognize(&self, args: &Vec<&str>) -> Option<Command> {
        let parser = self.cli_parser.as_ref().create();
        let matched = parser.get_matches_from(args);

        self.command_selector
            .as_ref()
            .select_by(matched)
    }
}
