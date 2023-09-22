use std::collections::HashMap;

use clap::{ArgMatches, Command};
use itertools::Itertools;

use crate::cli::cli_command::CliCommand;

pub(in crate::cli) struct ComposedCliCommand<'a, T> {
    commands: HashMap<&'a str, Box<dyn CliCommand<T>>>,
}

impl<'a, T, const N: usize> From<[(&'a str, Box<dyn CliCommand<T>>); N]> for ComposedCliCommand<'a, T> {
    fn from(cmds: [(&'a str, Box<dyn CliCommand<T>>); N]) -> Self {
        Self {
            commands: HashMap::from_iter(cmds),
        }
    }
}

impl<'a, T> ComposedCliCommand<'a, T> {
    pub(in crate::cli) fn blueprints(&self) -> Vec<Command> {
        self.commands.values().map(|c| c.blueprint()).collect_vec()
    }

    pub(in crate::cli) fn parse(&self, matches: &ArgMatches) -> Option<T> {
        matches.subcommand()
            .and_then(|(name, cmd_matches)| self.commands.get(name).and_then(|c| c.parse(cmd_matches)))
    }
}
