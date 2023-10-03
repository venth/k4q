use std::sync::Arc;

use ProgressRange::Limited;

use crate::domain::model::ProgressRange;
use crate::domain::ports;

pub(crate) async fn run_app<'a>(
    args: Vec<String>,
    command_recognizer: Arc<dyn ports::CommandRecognizer>,
    progress_starter: Arc<dyn ports::ProgressStarter<'a> + 'a>,
) {
    let progress = progress_starter.start("starts k4fq".to_owned(), Limited(1)).await;
    let recognized_command = command_recognizer.recognize(&args);
    progress.message(format!("command recognized {:#?}", recognized_command).as_str()).await;
}
