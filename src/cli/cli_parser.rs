use clap::{Arg, Command};
use shaku::{Component, Interface};

pub trait CliParserFactory: Interface {
    fn create<'a>(&self) -> Command<'a>;
}

#[derive(Component)]
#[shaku(interface = CliParserFactory)]
pub struct ClapCliParserFactory {

}

impl CliParserFactory for ClapCliParserFactory {
    fn create<'a>(&self) -> Command<'a>  {
        Command::new("kaf")
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
                        .multiple_values(true)
                        .min_values(1)
                        .long("topics")
                        .value_terminator(";"))
                    .subcommand(Command::new("key")
                        .arg_required_else_help(true)
                        .about("searches for records matching given key criteria against kafka topics")
                        .arg(Arg::new("criteria")
                            .possible_values(&["eq"])
                            .required(true))
                        .arg(Arg::new("keyValue")
                            .required(true))))
    }
}