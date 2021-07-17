use std::env::Args;

use crate::domain::app;

mod domain;
mod cli;
mod kafka;

fn main() {
    let record_finder = kafka::record_finder::new();
    let query_action_factory = domain::query::create_action_factory(record_finder);
    let action_recognizer = cli::port_action_recognizer::new(query_action_factory);
    let app = domain::app::new(action_recognizer);

    let cmd_args: Vec<String> = std::env::args()
        .collect();
    let args = cmd_args.iter().map(AsRef::as_ref).collect();
    app.run(&args)
}
