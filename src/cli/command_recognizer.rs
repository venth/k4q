use shaku;
use shaku::Component;

use crate::domain::command::Command;
use crate::domain::port;

#[derive(Component)]
#[shaku(interface = port::CommandRecognizer)]
pub struct CliCommandRecognizer {}

impl port::CommandRecognizer for CliCommandRecognizer {
    fn recognize(&self, args: &Vec<&str>) -> Option<Command> {
        todo!()
    }
}
