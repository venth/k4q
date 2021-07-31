use std::sync::Arc;

use shaku;
use shaku::Component;

use crate::cli::action_selector::ActionSelector;
use crate::cli::cli_parser::CliParserFactory;
use crate::domain::action::Action;
use crate::domain::port::ActionRecognizer;
use crate::domain::service::ActionFactory;

#[derive(Component)]
#[shaku(interface = ActionRecognizer)]
pub struct CliActionRecognizer {
    #[shaku(inject)]
    cli_parser: Arc<dyn CliParserFactory>,

    #[shaku(inject)]
    action_selector: Arc<dyn ActionSelector>,
}

impl ActionRecognizer for CliActionRecognizer {
    fn recognize<'a>(&self, args: &'a Vec<&'a str>) -> Option<Box<dyn Action + 'a>> {
        let parser = self.cli_parser.as_ref().create();
        let matched = parser.get_matches_from(args);

        self.action_selector.as_ref()
            .select_by(&matched)
    }
}

struct DummyAction<'a> {
    args: &'a Vec<&'a str>,
}

impl<'a> Action for DummyAction<'a> {
    fn execute(&self) {
        todo!()
    }
}