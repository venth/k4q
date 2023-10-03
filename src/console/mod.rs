use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::model::ProgressRange;
use crate::domain::ports::ProgressNotifier;
use crate::domain::ports::ProgressStarter;

mod progress;
mod channel_progress;

struct Adapter<'a> {
    progress_starter: Box<dyn ProgressStarter<'a>>,
}

pub(crate) fn new<'a>() -> Arc<dyn ProgressStarter<'a> + 'a> {
    Adapter::new(progress::new())
}

impl<'a> Adapter<'a> {
    fn new(progress_starter: Box<dyn ProgressStarter>) -> Arc<dyn ProgressStarter> {
        Arc::new(Adapter { progress_starter })
    }
}

#[async_trait]
impl<'a> ProgressStarter<'a> for Adapter<'a> {
    async fn start(&'a self, start_message: String, range: ProgressRange) -> Box<dyn ProgressNotifier + 'a> {
        self.progress_starter.start(start_message, range).await
    }
}
