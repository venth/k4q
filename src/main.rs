use std::sync::Arc;

use itertools::Itertools;

use crate::di::Startable;

mod domain;
mod cli;
mod console;
mod properties;
mod async_tools;
mod di;

#[tokio::main]
async fn main() {
    let cli = cli::new();
    let console = console::new();
    let startable_console: Arc<dyn Startable> = console.clone();
    let started_console = startable_console.start();
    domain::services::run_app(std::env::args().collect_vec(), cli.clone(), console.clone()).await;
    started_console.await.unwrap();
}
