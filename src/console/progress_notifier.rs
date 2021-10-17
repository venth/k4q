use std::thread;
use std::time::Duration;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use shaku;

use crate::domain::ports;

impl ports::ProgressNotifier for ConsoleErrorNotifier {
    fn notify(&self, message: &str) {
        let m = MultiProgress::new();
        let sty = ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:100.cyan/blue} {pos:>11}/{len:11} {msg}")
            .progress_chars("##-");
        let pb = m.add(ProgressBar::new(128));
        pb.set_style(sty.clone());
        pb.set_position(0);
        let pb1 = m.add(ProgressBar::new(128));
        pb1.set_style(sty.clone());
        pb1.set_position(0);
        for i in 0..128 {
            pb.set_message(format!("item #{}", i + 1));
            pb.println(message);
            pb.inc(1);
            if i % 10 == 0 {
                pb1.inc(10);
            }
            thread::sleep(Duration::from_millis(15));
        }
        pb.finish();
        pb.finish_and_clear();
        pb1.finish_and_clear();
    }
}

#[derive(shaku::Component)]
#[shaku(interface = ports::ProgressNotifier)]
pub struct ConsoleErrorNotifier {}