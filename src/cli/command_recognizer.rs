use shaku;
use shaku::Component;

use crate::domain::command::Command;
use crate::domain::ports;

#[derive(Component)]
#[shaku(interface = ports::CommandRecognizer)]
pub struct CliCommandRecognizer {}

impl ports::CommandRecognizer for CliCommandRecognizer {
    fn recognize(&self, args: &Vec<&str>) -> Option<Command> {
        todo!()
    }
}
