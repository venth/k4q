use clap::{App, AppSettings, Arg};
use shaku::{Component, Interface};

pub trait CliParserFactory: Interface {
    fn create<'a>(&self) -> App<'a>;
}

#[derive(Component)]
#[shaku(interface = CliParserFactory)]
pub struct ClapCliParserFactory {

}

impl CliParserFactory for ClapCliParserFactory {
    fn create<'a>(&self) -> App<'a>  {
        App::new("kaf")
            .setting(AppSettings::ArgRequiredElseHelp)
            .setting(AppSettings::GlobalVersion)
            .version("0.0.1")
            .author("Artur Krysiak <artur.krysiak.warszawa@gmail.com>")
            .about("Interacts with kafka from command line")
            .subcommand(
                App::new("query")
                    .about("searches by given criteria against kafka topics")
                    .arg(Arg::new("topics")
                        .required(true)
                        .multiple_values(true)
                        .min_values(1))
                    .subcommand(App::new("key")
                        .setting(AppSettings::ArgRequiredElseHelp)
                        .about("searches for records matching given key criteria against kafka topics")
                        .arg(Arg::new("criteria")
                            .possible_values(&["eq"])
                            .required(true))
                        .arg(Arg::new("keyValue")
                            .required(true))))
    }
}