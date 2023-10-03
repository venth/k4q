use async_trait::async_trait;

use crate::domain::model::ProgressRange;

#[async_trait]
pub(crate) trait ProgressStarter<'a>: Send + Sync {
    async fn start(&'a self, start_message: String, range: ProgressRange) -> Box<dyn ProgressNotifier + 'a>;
}

#[async_trait]
pub(crate) trait ProgressNotifier: Send + Sync {

    async fn step(&self);
    async fn step_with_message(&self, message: &str);
    async fn message(&self, msg: &str);
}
