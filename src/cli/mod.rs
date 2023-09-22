use crate::domain::{model as domainModel, ports};

mod cli_command_recognizer;
mod properties_location_provider;
mod command;
mod cli_command;
mod composed_cli_command;

struct Adapter {
    command_recognizer: Box<dyn ports::CommandRecognizer>,
}

pub(crate) fn new() -> impl ports::CommandRecognizer {
    return Adapter::new(cli_command_recognizer::new());
}

impl Adapter {
    fn new(command_recognizer: impl ports::CommandRecognizer + 'static)
           -> impl ports::CommandRecognizer {
        return Adapter {
            command_recognizer: Box::new(command_recognizer),
        };
    }
}

impl ports::CommandRecognizer for Adapter {
    fn recognize(&self, args: &Vec<String>) -> domainModel::RecognizedCommand {
        return self.command_recognizer
            .recognize(args);
    }
}

#[cfg(test)]
mod tests {
    use mockall::mock;

    use domainModel::RecognizedCommand;

    use crate::cli::Adapter;
    use crate::domain::model as domainModel;
    use crate::domain::ports;
    use crate::domain::ports::CommandRecognizer;

    mock! {
        CliCommandRecognizer {}
        impl ports::CommandRecognizer for CliCommandRecognizer {
            fn recognize(&self, args: & Vec<String>) -> domainModel::RecognizedCommand;
        }
    }

    #[test]
    fn delegates_recognition() {
        // given
        let some_args = args(&["some_arg1", "some_arg2"]);
        let recognized_command = RecognizedCommand::Unrecognized("unrecognized".to_string());

        #[warn(unused_mut)]
        let mut recognizer = MockCliCommandRecognizer::new();
        let mock_blueprint = recognized_command.clone();
        recognizer.expect_recognize().times(1).returning(move |_| mock_blueprint.clone());
        let adapter = Adapter::new(recognizer);

        // when
        let _result = adapter.recognize(&some_args);

        // then
        assert!(matches!(recognized_command, _result))
    }

    fn args(args: &[&str]) -> Vec<String> {
        return args.into_iter().map(|s| s.to_string()).collect();
    }
}
