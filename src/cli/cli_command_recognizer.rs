use clap::Command;

use crate::cli::command;
use crate::cli::composed_cli_command::ComposedCliCommand;
use crate::domain::model as domainModel;
use crate::domain::ports;

struct CliCommandRecognizer<'a> {
    commands: ComposedCliCommand<'a, domainModel::RecognizedCommand>,
}

pub(in crate::cli) fn new() -> impl ports::CommandRecognizer {
    let commands = ComposedCliCommand::from([
        command::QueryCommand::new(),
    ]);
    return CliCommandRecognizer::new(commands);
}

impl<'a> ports::CommandRecognizer for CliCommandRecognizer<'a> {
    fn recognize(&self, args: &Vec<String>) -> domainModel::RecognizedCommand {
        self.k4fq_command()
            .try_get_matches_from(args)
            .map_err(|e| e.render().to_string())
            .and_then(|m| self.commands.parse(&m).ok_or_else(|| "no command was recognized".to_string()))
            .unwrap_or_else(domainModel::RecognizedCommand::Unrecognized)
    }
}

impl<'a> CliCommandRecognizer<'a> {
    fn new(commands: ComposedCliCommand<'a, domainModel::RecognizedCommand>) -> Self {
        Self { commands }
    }

    fn k4fq_command(&self) -> Command {
        Command::new("k4fq")
            .arg_required_else_help(true)
            .propagate_version(true)
            .version("0.0.1")
            .author("Artur Krysiak <artur.krysiak.warszawa@gmail.com>")
            .about("Interacts with kafka from command line")
            .subcommands(self.commands.blueprints())
    }
}

#[cfg(test)]
mod test {
    use mockall::mock;

    use cli::cli_command_recognizer;
    use domain::model as domainModel;
    use domain::ports::CommandRecognizer;

    use crate::cli;
    use crate::cli::cli_command::CliCommand;
    use crate::cli::composed_cli_command::ComposedCliCommand;
    use crate::domain;
    use crate::domain::model::{RecognizedCriteria, RecognizedQueryTerm};

    #[test]
    fn recognizes_query_by_key() {
        // given
        let some_args = args(&["k4fq", "query", "--topics", "topic1", "topic2", ";", "key", "eq", "123"]);

        // and
        let recognizer = cli_command_recognizer::new();

        // when
        let result = recognizer.recognize(&some_args);

        // then
        assert!(matches!(result, domainModel::RecognizedCommand::QueryByKey(_, _)));
    }

    #[test]
    fn returns_query_help_as_unrecognised() {
        // given
        let some_args = args(&["k4fq", "query", "--help"]);

        // and
        let recognizer = cli_command_recognizer::new();

        // when
        let result = recognizer.recognize(&some_args);

        // then
        assert!(matches!(result, domainModel::RecognizedCommand::Unrecognized(_)));
    }

    #[test]
    fn returns_help_description_as_unrecognised() {
        // given
        let some_args = args(&["k4fq", "--help"]);

        // and
        let recognizer = cli_command_recognizer::new();

        // when
        let result = recognizer.recognize(&some_args);

        // then
        assert!(matches!(result, domainModel::RecognizedCommand::Unrecognized(_)));
    }

    #[test]
    fn does_not_recognise_unsupported_command() {
        // given
        let supported_commands = ComposedCliCommand::from([
            some_command("supported")
        ]);

        // and
        let recognizer = cli_command_recognizer::CliCommandRecognizer::new(supported_commands);

        // when
        let unsupported_command_name = "unsupported_command";
        let result = recognizer.recognize(&args(&["k4fq", unsupported_command_name]));

        // then
        assert!(matches!(result, domainModel::RecognizedCommand::Unrecognized(err)
            if err.contains(format!("error: unrecognized subcommand '{unsupported_command_name}'").as_str())));
    }

    #[test]
    fn recognise_supported_command() {
        // given
        let supported_command_name = "supported_command";
        let _command_result = domainModel::RecognizedCommand::QueryByKey(domainModel::RecognizedTopicPattern::Direct(vec![]), RecognizedQueryTerm::Key(RecognizedCriteria::Eq("".to_string())));
        let supported_commands = ComposedCliCommand::from([
            command_of(supported_command_name.as_ref(), _command_result.clone()),
            some_command("just_some_command"),
        ]);

        // and
        let recognizer = cli_command_recognizer::CliCommandRecognizer::new(supported_commands);

        // when
        let result = recognizer.recognize(&args(&["k4fq", supported_command_name]));

        // then
        assert!(matches!(result, _command_result));
    }

    mock! {
        Command {}
        impl CliCommand<domainModel::RecognizedCommand> for Command {
            fn blueprint(&self) -> clap::Command;
            fn parse(&self, matches: &clap::ArgMatches) -> Option<domainModel::RecognizedCommand>;
        }
    }

    fn command_of(name: &str, result: domainModel::RecognizedCommand) -> (&str, Box<dyn CliCommand<domainModel::RecognizedCommand>>) {
        let mut command = MockCommand::new();
        let mocked_name = name.to_string();
        command.expect_blueprint().times(1).returning(move || {
            clap::Command::new(mocked_name.clone())
        });

        command.expect_parse().return_once(move |_| Some(result));

        (name, Box::new(command))
    }

    fn some_command(name: &str) -> (&str, Box<dyn CliCommand<domainModel::RecognizedCommand>>) {
        command_of(name, domainModel::RecognizedCommand::Unrecognized("supported".to_string()))
    }

    fn args(args: &[&str]) -> Vec<String> {
        return args.into_iter().map(|s| s.to_string()).collect();
    }
}
