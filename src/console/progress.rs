use std::sync::{Arc, Weak};

use async_trait::async_trait;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use crate::domain::model::ProgressRange;
use crate::domain::ports::{ProgressNotifier, ProgressStarter};

pub(in crate::console) fn new() -> Arc<dyn ProgressStarter> {
    IndicatifProgressStarter::new(MultiProgress::new())
}

struct IndicatifProgressStarter {
    bar: MultiProgress,
}

impl Observer for IndicatifProgressStarter {
    fn notify(&self, bar: &ProgressBar) {
        bar.finish();
        self.bar.remove(bar);
    }
}

impl IndicatifProgressStarter {
    fn new(bar: MultiProgress) -> Arc<dyn ProgressStarter> {
        Arc::new(IndicatifProgressStarter { bar })
    }
}


#[async_trait]
impl ProgressStarter for IndicatifProgressStarter {
    async fn start(self: Arc<Self>, start_message: String, range: ProgressRange) -> Arc<dyn ProgressNotifier> {
        let self_observer = Arc::clone(&self);
        let observer = Arc::downgrade(&self_observer);
        let progress = self_observer.bar.add(new_progress_bar(start_message.as_ref(), range));
        CurrentProgress::new(observer, progress)
    }
}

fn new_progress_bar(msg: &str, range: ProgressRange) -> ProgressBar {
    let len = match range { ProgressRange::Limited(len) => { len } };
    let bar = ProgressBar::new(len);
    let style = ProgressStyle::with_template("{spinner} {wide_bar} {pos}/{len} {msg}")
        .unwrap()
        .progress_chars("##-");
    bar.set_style(style);
    bar.set_message(msg.to_owned());
    bar
}

trait Observer: Sync {
    fn notify(&self, bar: &ProgressBar);
}

struct CurrentProgress {
    observer: Weak<IndicatifProgressStarter>,
    bar: ProgressBar,
}

impl Drop for CurrentProgress {
    fn drop(&mut self) {
        self.observer.upgrade().map(|o| o.notify(&(self.bar)));
    }
}

impl CurrentProgress {
    fn new(observer: Weak<IndicatifProgressStarter>, bar: ProgressBar) -> Arc<dyn ProgressNotifier> {
        Arc::new(
            CurrentProgress {
                observer,
                bar,
            }
        )
    }
}

#[async_trait]
impl ProgressNotifier for CurrentProgress {
    async fn step(&self) {
        self.bar.inc(1);
    }

    async fn step_with_message(&self, message: &str) {
        self.bar.inc(1);
        self.bar.set_message(message.to_owned());
    }

    async fn message(&self, msg: &str) {
        self.bar.println(msg);
    }
}
