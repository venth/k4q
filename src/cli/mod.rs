use clap::{App, AppSettings, Arg, ArgMatches};

pub fn app() -> ArgMatches {
    App::new("kaf")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::GlobalVersion)
        .version("0.0.1")
        .author("Artur Krysiak <artur.krysiak.warszawa@gmail.com>")
        .about("Interacts with kafka from command line")
        .subcommand(
            App::new("query")
                .about("searches by given criteria against kafka topics")
                .subcommand(App::new("key")
                    .setting(AppSettings::ArgRequiredElseHelp)
                    .about("searches for records matching given key criteria against kafka topics")
                    .arg(Arg::new("operator")
                        .possible_values(&["eq"])
                        .required(true))
                    .arg(Arg::new("value")
                        .required(true))
                    .arg(Arg::new("topics")
                        .required(true)
                        .min_values(1))))
        .get_matches()
}
