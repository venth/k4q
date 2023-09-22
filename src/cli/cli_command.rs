pub(in crate::cli) trait CliCommand<T> {
    fn blueprint(&self) -> clap::Command;
    fn parse(&self, matches: &clap::ArgMatches) -> Option<T>;
}
