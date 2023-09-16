mod domain;
mod cli;
mod console;
mod properties;
mod iter;

#[tokio::main]
async fn main() {
    let _cmd_args: Vec<String> = std::env::args().collect();
    let cli_adapter = cli::new();
    let run_app = domain::services::new_app_runner(&cli_adapter);
    run_app();
}
