use crate::cli::cli_configuration;
use crate::cli::query_matcher::{key_equlas_value, QueryMatcher};
use crate::domain::action::{Action, ActionFactory};
use crate::domain::port::ActionRecognizer;

pub fn new(action_factory: Box<dyn ActionFactory>) -> Box<dyn ActionRecognizer> {
    let query_matcher = key_equlas_value::new(action_factory);
    Box::from(CliActionRecognizer{ query_matcher })
}

impl ActionRecognizer for CliActionRecognizer {
    fn recognize(&self, args: &Vec<&str>) -> Option<Box<dyn Action>> {
        let matched = cli_configuration::app()
            .get_matches_from(args);

        self.query_matcher.matches(&matched)
    }
}

struct CliActionRecognizer {
    query_matcher: Box<dyn QueryMatcher>,
}
