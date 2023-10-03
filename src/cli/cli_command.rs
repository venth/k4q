pub(in crate::cli) trait CliCommand<T>: Send + Sync {
    fn blueprint(&self) -> clap::Command;
    fn parse(&self, matches: &clap::ArgMatches) -> Option<T>;
}
