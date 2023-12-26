use std::sync::Arc;
use async_trait::async_trait;

use crate::domain::model::ProgressRange;

#[async_trait]
pub(crate) trait ProgressStarter: Send + Sync {
    async fn start(self: Arc<Self>, start_message: String, range: ProgressRange) -> Arc<dyn ProgressNotifier>;
}

#[async_trait]
pub(crate) trait ProgressNotifier: Send + Sync {

    async fn step(&self);
    async fn step_with_message(&self, message: &str);
    async fn message(&self, msg: &str);
}
