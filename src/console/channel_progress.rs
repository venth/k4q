use std::collections::HashMap;
use std::ops::DerefMut;
use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::{Mutex, MutexGuard};
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tokio::task::JoinHandle;
use uuid::Uuid;

use crate::domain::model::ProgressRange;
use crate::domain::ports::{ProgressNotifier, ProgressStarter};

pub(super) fn new(progress_starter: Arc<dyn ProgressStarter>) -> (Arc<ChanneledStarter>, Arc<Mutex<ChanneledListener>>) {
    let (sender, receiver) = mpsc::channel(100);
    (
        ChanneledStarter::new(Box::new(Uuid::new_v4), sender),
        Arc::new(Mutex::new(ChanneledListener::new(receiver, progress_starter)))
    )
}

pub(in crate::console) struct ChanneledStarter {
    id_supplier: Box<dyn Fn() -> Uuid + Sync + Send>,
    sender: Sender<ConsoleCommands>,
}

pub(in crate::console) struct ChanneledListener {
    receiver: Receiver<ConsoleCommands>,
    progress_starter: Arc<dyn ProgressStarter>,
    running: HashMap<Uuid, Arc<dyn ProgressNotifier>>,
}

struct ChanneledNotifier {
    progress_id: Uuid,
    sender: Sender<ConsoleCommands>,
}

impl ChanneledStarter {
    fn new(id_supplier: Box<dyn Fn() -> Uuid + Sync + Send>, sender: Sender<ConsoleCommands>) -> Arc<Self> {
        Arc::new(ChanneledStarter { id_supplier, sender })
    }

    pub(in crate::console) async fn finish(self: Arc<Self>) {
        self.sender.send(ConsoleCommands::Terminate).await.unwrap()
    }
}

impl ChanneledNotifier {
    fn new(progress_id: Uuid, sender: Sender<ConsoleCommands>) -> Arc<dyn ProgressNotifier> {
        Arc::new(ChanneledNotifier { progress_id, sender })
    }
}

impl ChanneledListener {
    fn new(receiver: Receiver<ConsoleCommands>, progress_starter: Arc<dyn ProgressStarter>) -> Self {
        ChanneledListener {
            receiver,
            progress_starter,
            running: Default::default(),
        }
    }

    pub(in crate::console) fn start(this: Arc<Mutex<Self>>) -> JoinHandle<()> {
        tokio::task::spawn(async move {
            loop {
                let attempt = this.try_lock();
                let stop = match attempt {
                    None => {
                        false
                    }
                    Some(locked) => { process_command(locked).await }
                };
                if stop {
                    break;
                }
            }
        })
    }
}

fn process_command<'a>(mut locked: MutexGuard<'a, ChanneledListener>) -> impl futures::Future<Output=bool> + Send + 'a {
    async move {
        let l = locked.deref_mut();
        let receiver = &mut (l.receiver);
        let progress_starter = &(l.progress_starter);
        let running = &mut (l.running);

        return match receiver.try_recv() {
            Ok(cmd) => {
                match cmd {
                    ConsoleCommands::Start(id, msg, range) => {
                        let started = Arc::clone(progress_starter).start(msg, range).await;
                        running.insert(id, started);
                        false
                    }
                    ConsoleCommands::Step(progress_id) => {
                        running.get(&progress_id)
                            .map(|p| p.step())
                            .unwrap().await;
                        false
                    }
                    ConsoleCommands::StepWithMessage(progress_id, msg) => {
                        running.get(&progress_id)
                            .map(|p| p.step_with_message(msg.as_str()))
                            .unwrap().await;
                        false
                    }
                    ConsoleCommands::Message(progress_id, msg) => {
                        running.get(&progress_id)
                            .map(|p| p.message(msg.as_str()))
                            .unwrap().await;
                        false
                    }
                    ConsoleCommands::Stop(progress_id) => {
                        running.remove(&progress_id);
                        false
                    }
                    ConsoleCommands::Terminate => { true }
                }
            }
            Err(e) => {
                match e {
                    TryRecvError::Empty => { false }
                    TryRecvError::Disconnected => {
                        true
                    }
                }
            }
        };
    }
}


#[async_trait]
impl ProgressStarter for ChanneledStarter {
    async fn start(self: Arc<Self>, start_message: String, range: ProgressRange<>) -> Arc<dyn ProgressNotifier> {
        let sender = self.sender.clone();
        let progress_id = (self.id_supplier)();
        sender.send(ConsoleCommands::Start(progress_id, start_message.to_owned(), range)).await
            .expect("the command to be sent");
        ChanneledNotifier::new(progress_id, sender.clone())
    }
}

impl Drop for ChanneledNotifier {
    fn drop(&mut self) {
        futures::executor::block_on(self.sender.send(ConsoleCommands::Stop(self.progress_id))).unwrap();
    }
}

#[async_trait]
impl<'a> ProgressNotifier for ChanneledNotifier {
    async fn step(&self) {
        self.sender.send(ConsoleCommands::Step(self.progress_id)).await.expect("message must be sent");
    }

    async fn step_with_message(&self, message: &str) {
        self.sender.send(ConsoleCommands::StepWithMessage(self.progress_id, message.to_owned())).await.expect("message must be sent");
    }

    async fn message(&self, msg: &str) {
        self.sender.send(ConsoleCommands::Message(self.progress_id, msg.to_owned())).await.expect("message must be sent");
    }
}

#[derive(Debug)]
enum ConsoleCommands {
    Start(Uuid, String, ProgressRange),
    Step(Uuid),
    StepWithMessage(Uuid, String),
    Message(Uuid, String),
    Stop(Uuid),
    Terminate,
}

#[cfg(test)]
mod test {
    use futures::executor::block_on;
    use tokio::sync::mpsc;
    use uuid::Uuid;

    use crate::console::channel_progress::ChanneledStarter;
    use crate::console::channel_progress::ConsoleCommands::Start;
    use crate::domain::model::ProgressRange;
    use crate::domain::ports::ProgressStarter;

    #[test]
    fn sends_start_command_on_start_request() {
        // given
        let progress_id = Uuid::new_v4();

        // and
        let (sender, mut receiver) = mpsc::channel(1);
        let mocked_id = progress_id.clone();
        let progress_starter = ChanneledStarter::new(Box::new(move || mocked_id), sender);

        // and
        let start_msg = "started progress";
        let progress_range = ProgressRange::Limited(1);
        let progress = progress_starter.start(start_msg.into(), progress_range);

        // and
        let received = receiver.recv();

        _ = block_on(progress);
        let received_result = block_on(received);

        // then
        assert_eq!(received_result.is_some(), true);
        assert!(matches!(received_result.unwrap(), Start(progress_id, start_msg, progress_range)));
    }
}
