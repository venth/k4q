use std::collections::HashMap;
use std::ops::DerefMut;
use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::{Mutex, MutexGuard};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tokio::task::JoinHandle;
use uuid::Uuid;

use crate::domain::model::ProgressRange;
use crate::domain::ports::{ProgressNotifier, ProgressStarter};

pub(super) fn new<'a>() -> Box<dyn ProgressStarter<'a> + 'a> {
    let (sender, receiver) = mpsc::channel(100);
    ChanneledStarter::new(Box::new(Uuid::new_v4), sender)
}

struct ChanneledStarter {
    id_supplier: Box<dyn Fn() -> Uuid + Sync + Send>,
    sender: Sender<ConsoleCommands>,
}

struct ChanneledListener<'a> {
    receiver: Receiver<ConsoleCommands>,
    progress_starter: Box<dyn ProgressStarter<'a>>,
    running: HashMap<Uuid, Box<dyn ProgressNotifier + 'a>>,
}

struct ChanneledNotifier {
    progress_id: Uuid,
    sender: Sender<ConsoleCommands>,
}

impl ChanneledStarter {
    fn new<'a>(id_supplier: Box<dyn Fn() -> Uuid + Sync + Send>, sender: Sender<ConsoleCommands>) -> Box<dyn ProgressStarter<'a> + 'a> {
        Box::new(ChanneledStarter { id_supplier, sender })
    }
}

impl ChanneledNotifier {
    fn new<'a>(progress_id: Uuid, sender: Sender<ConsoleCommands>) -> Box<dyn ProgressNotifier + 'a> {
        Box::new(ChanneledNotifier { progress_id, sender })
    }
}

impl<'a> ChanneledListener<'a> {
    fn start(this: Arc<Mutex<Self>>) -> JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                let attempt = this.try_lock();
                let stop = match attempt {
                    None => { false }
                    Some(mut locked) => { process_command(&mut locked).await }
                };
                if stop {
                    break;
                }
            }
        })
    }
}

fn process_command<'a>(locked: &'a mut MutexGuard<'a, ChanneledListener<'a>>) -> impl futures::Future<Output=bool> + Send + 'a {
    async {
        let l = locked.deref_mut();
        let receiver = &mut (l.receiver);
        let progress_starter = &(l.progress_starter);
        let running = &mut (l.running);

        return match receiver.try_recv() {
            Ok(cmd) => {
                match cmd {
                    ConsoleCommands::Start(id, msg, range) => {
                        let started = progress_starter.start(msg, range).await;
                        running.insert(id, started);
                    }
                };
                false
            }
            Err(_) => {
                true
            }
    };

    }
}


#[async_trait]
impl<'a> ProgressStarter<'a> for ChanneledStarter {
    async fn start(&'a self, start_message: String, range: ProgressRange<>) -> Box<dyn ProgressNotifier + 'a> {
        let sender = self.sender.clone();
        let progress_id = (self.id_supplier)();
        sender.send(ConsoleCommands::Start(progress_id, start_message.to_owned(), range)).await
            .expect("the command to be sent");
        ChanneledNotifier::new(progress_id, self.sender.clone())
    }
}

#[async_trait]
impl<'a> ProgressNotifier for ChanneledNotifier {
    async fn step(&self) {
        todo!()
    }

    async fn step_with_message(&self, message: &str) {
        todo!()
    }

    async fn message(&self, msg: &str) {
        todo!()
    }
}

#[derive(Debug)]
enum ConsoleCommands {
    Start(Uuid, String, ProgressRange)
}

#[cfg(test)]
mod test {
    use futures::executor::block_on;
    use tokio::sync::mpsc;
    use uuid::Uuid;

    use crate::console::channel_progress::ChanneledStarter;
    use crate::console::channel_progress::ConsoleCommands::Start;
    use crate::domain::model::ProgressRange;

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
