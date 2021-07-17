use clap::{App, AppSettings, Arg};

pub fn app<'a>() -> App<'a> {
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
