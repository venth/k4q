use clap::{Arg, Command};
use clap::builder::PossibleValuesParser;

pub trait CliParserFactory {
    fn create(&self) -> Command;
}

pub struct ClapCliParserFactory {}

impl CliParserFactory for ClapCliParserFactory {
    fn create(&self) -> Command {
        Command::new("k4fq")
            .arg_required_else_help(true)
            .propagate_version(true)
            .version("0.0.1")
            .author("Artur Krysiak <artur.krysiak.warszawa@gmail.com>")
            .about("Interacts with kafka from command line")
            .subcommand(
                Command::new("query")
                    .about("searches by given criteria against kafka topics")
                    .arg_required_else_help(true)
                    .arg(Arg::new("topics")
                        .required(true)
                        .num_args(1..)
                        .long("topics")
                        .value_terminator(";"))
                    .subcommand(Command::new("key")
                        .arg_required_else_help(true)
                        .about("searches for records matching given key criteria against kafka topics")
                        .arg(Arg::new("criteria")
                            .value_parser(PossibleValuesParser::new(["eq"]))
                            .required(true))
                        .arg(Arg::new("keyValue")
                            .required(true))))
    }
}