use shaku;

use crate::domain::ports;

impl ports::ErrorNotifier for ConsoleErrorNotifier {
    fn notify(&self, message: &str) {
        todo!()
    }
}

#[derive(shaku::Component)]
#[shaku(interface = ports::ErrorNotifier)]
pub struct ConsoleErrorNotifier {}