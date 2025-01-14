use std::sync::Arc;

use indicatif::{MultiProgress, ProgressBar, ProgressDrawTarget, ProgressStyle};
use shaku;

use crate::domain::{model, ports};

impl ports::ProgressNotifier for ConsoleErrorNotifier {
    fn notify(&self, _: &str) {}

    fn start(&self, estimated_max_size: &model::Count) -> Arc<dyn model::Progress + Sync + Send> {
        let pb = self.progress.add(ProgressBar::new(estimated_max_size.value));
        pb.set_style(self.progress_style.clone());
        pb.set_position(0);
        pb.set_draw_target(ProgressDrawTarget::stderr());

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