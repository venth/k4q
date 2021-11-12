mod domain;
mod cli;
mod kafka;
mod console;
mod properties;
mod monads;

#[tokio::main]
async fn main() {
    let app_module = domain::module(
        cli::module(),
        kafka::module(),
        console::module(),
    properties::module());

    let app: &dyn domain::service::App = app_module.resolve_ref();

    let cmd_args: Vec<String> = std::env::args().collect();
    let args = cmd_args.iter().map(AsRef::as_ref).collect();

    app.run(&args);
}
