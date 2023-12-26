use std::sync::Arc;
use tokio::task::JoinHandle;

pub(crate) trait Startable {
    fn start(self: Arc<Self>) -> JoinHandle<()>;
}
