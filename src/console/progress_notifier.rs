use std::thread;
use std::time::Duration;

use indicatif::{MultiProgress, ProgressBar, ProgressDrawTarget, ProgressStyle};
use shaku;

use crate::domain::ports;

impl ports::ProgressNotifier for ConsoleErrorNotifier {
    fn notify(&self, message: &str) {
        let m = MultiProgress::new();
        let sty = ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .progress_chars("##-");
        let pb = m.add(ProgressBar::new(128));
        pb.set_style(sty);
        pb.set_position(0);
        for i in 0..128 {
            pb.set_message(format!("item #{}", i + 1));
            pb.println(message);
            pb.inc(1);
            thread::sleep(Duration::from_millis(15));
        }
        pb.println(message);
        pb.finish();
    }
}

#[derive(shaku::Component)]
#[shaku(interface = ports::ProgressNotifier)]
pub struct ConsoleErrorNotifier {}