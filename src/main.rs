mod domain;
mod cli;

fn main() {
    let action_recognizer = cli::action_recognizer::new();
    let app = domain::app::new(&action_recognizer);

    let cmd_args: Vec<String> = std::env::args().collect();
    let args = cmd_args.iter().map(AsRef::as_ref).collect();

    app.run(&args);
}
