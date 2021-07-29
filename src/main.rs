use shaku::HasComponent;

mod domain;
mod cli;

fn main() {
    let app_module = domain::domain_module(cli::cli_module());
    let app: &dyn domain::service::App = app_module.resolve_ref();

    let cmd_args: Vec<String> = std::env::args().collect();
    let args = cmd_args.iter().map(AsRef::as_ref).collect();

    app.run(&args);
}
