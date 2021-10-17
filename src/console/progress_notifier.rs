use std::sync::Arc;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use shaku;

use crate::domain::{model, ports};

impl ports::ProgressNotifier for ConsoleErrorNotifier {
    fn notify(&self, message: &str) {}

    fn start(&self) -> Arc<dyn model::Progress> {
        let pb = self.progress.add(ProgressBar::new(4280));
        pb.set_style(self.progress_style.clone());
        pb.set_position(0);

        Arc::new(IndicatifProgress {
            bar: pb
        })
    }
}

struct IndicatifProgress {
    bar: ProgressBar,
}

impl<'a> model::Progress for IndicatifProgress {
    fn message(&self, msg: &str) {
        self.bar.println(msg);
    }

    fn increase(&self) {
        self.bar.inc(1);
    }

    fn finish(&self) {
        self.bar.finish_and_clear();
    }
}


#[derive(shaku::Component)]
#[shaku(interface = ports::ProgressNotifier)]
pub struct ConsoleErrorNotifier {
    progress: MultiProgress,
    progress_style: ProgressStyle,
}