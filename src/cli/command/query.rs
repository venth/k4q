use clap::{Arg, ArgMatches, Command};

use crate::cli::cli_command::CliCommand;
use crate::cli::composed_cli_command::ComposedCliCommand;
use crate::domain::model::{RecognizedCommand, RecognizedCriteria, RecognizedQueryTerm, RecognizedTopicPattern};

pub(in crate::cli) struct QueryCommand<'a> {
    composed: ComposedCliCommand<'a, RecognizedQueryTerm>,
}

impl<'a> QueryCommand<'a> {
    pub(in crate::cli) fn new() -> (&'a str, Box<dyn CliCommand<RecognizedCommand>>) {
        let composed = ComposedCliCommand::from([QueryKey::new()]);
        ("query", Box::new(QueryCommand { composed }))
    }
}

impl<'a> CliCommand<RecognizedCommand> for QueryCommand<'a> {
    fn blueprint(&self) -> Command {
        let cmd = Command::new("query")
            .about("searches by given criteria against kafka topics")
            .arg_required_else_help(true)
            .arg(Arg::new("topics")
                .required(true)
                .num_args(1..)
                .long("topics")
                .value_terminator(";"));

        cmd.subcommands(self.composed.blueprints())
    }

    fn parse(&self, matches: &ArgMatches) -> Option<RecognizedCommand> {
        let topic_patterns = matches.get_many::<String>("topics")
            .map(|t| t.map(|tt| tt.to_owned()))
            .map(Vec::from_iter);

        topic_patterns
            .map(|p| RecognizedTopicPattern::Direct(p))
            .zip(self.composed.parse(matches))
            .map(|(topic_pattern, query_term)| RecognizedCommand::QueryByKey(topic_pattern, query_term))
    }
}

struct QueryKey<'a> {
    composed: ComposedCliCommand<'a, RecognizedCriteria>,
}

impl<'a> QueryKey<'a> {
    fn new() -> (&'a str, Box<dyn CliCommand<RecognizedQueryTerm> + 'a>) {
        ("key", Box::new(Self {
            composed: ComposedCliCommand::from([
                Eq::new(),
            ])
        }))
    }
}

impl<'a> CliCommand<RecognizedQueryTerm> for QueryKey<'a> {
    fn blueprint(&self) -> Command {
        Command::new("key")
            .arg_required_else_help(true)
            .about("searches for records matching given key criteria against kafka topics")
            .subcommands(self.composed.blueprints())
    }

    fn parse(&self, matches: &ArgMatches) -> Option<RecognizedQueryTerm> {
        self.composed.parse(matches).map(RecognizedQueryTerm::Key)
    }
}

struct Eq {}

impl Eq {
    fn new<'a>() -> (&'a str, Box<dyn CliCommand<RecognizedCriteria>>) { ("eq", Box::new(Self {})) }
}

impl CliCommand<RecognizedCriteria> for Eq {
    fn blueprint(&self) -> Command {
        Command::new("eq")
            .arg(Arg::new("keyValue")
                .required(true))
    }

    fn parse(&self, matches: &ArgMatches) -> Option<RecognizedCriteria> {
        matches.get_one::<String>("keyValue")
            .map(|key_value| RecognizedCriteria::Eq(key_value.to_string()))
    }
}
