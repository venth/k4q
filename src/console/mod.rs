use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::Mutex;
use tokio::task::JoinHandle;

use crate::console::channel_progress::{ChanneledListener, ChanneledStarter};
use crate::di::Startable;
use crate::domain::model::ProgressRange;
use crate::domain::ports::ProgressNotifier;
use crate::domain::ports::ProgressStarter;

mod progress;
mod channel_progress;

pub(crate) struct Adapter {
    progress_starter: Arc<ChanneledStarter>,
    channeled_listener: Arc<Mutex<ChanneledListener>>,
}

pub(crate) fn new() -> Arc<Adapter> {
    let (starter, listener) = channel_progress::new(progress::new());
    Arc::new(Adapter::new(starter, listener))
}

impl Adapter {
    fn new(progress_starter: Arc<ChanneledStarter>,
           channeled_listener: Arc<Mutex<ChanneledListener>>) -> Self {
        Adapter {
            progress_starter: Arc::clone(&progress_starter),
            channeled_listener,
        }
    }

    pub(in crate::console) async fn finish(self: Arc<Self>) {
        self.progress_starter.clone().finish().await;
    }
}

impl Startable for Adapter {
    fn start(self: Arc<Self>) -> JoinHandle<()> {
        let channeled_listener = Arc::clone(&self.channeled_listener);
        ChanneledListener::start(channeled_listener)
    }
}

#[async_trait]
impl ProgressStarter for Adapter {
    async fn start(self: Arc<Self>, start_message: String, range: ProgressRange) -> Arc<dyn ProgressNotifier> {
        let starter = Arc::clone(&self.progress_starter);
        starter.start(start_message, range).await
    }
}
