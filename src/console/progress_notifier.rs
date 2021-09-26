use shaku;

use crate::domain::ports;

impl ports::ProgressNotifier for ConsoleErrorNotifier {
    fn notify(&self, message: &str) {
        todo!()
    }
}

#[derive(shaku::Component)]
#[shaku(interface = ports::ProgressNotifier)]
pub struct ConsoleErrorNotifier {}