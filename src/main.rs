use itertools::Itertools;

mod domain;
mod cli;
mod console;
mod properties;
mod async_tools;

#[tokio::main]
async fn main() {
    let cli = cli::new();
    let console = console::new();
    domain::services::run_app(std::env::args().collect_vec(), cli.clone(), console.clone()).await;
}
