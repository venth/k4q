use std::sync::{Arc, Weak};

use async_trait::async_trait;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use crate::domain::model::ProgressRange;
use crate::domain::ports::{ProgressNotifier, ProgressStarter};

pub(in crate::console) fn new<'a>() -> Box<dyn ProgressStarter<'a> + 'a> {
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
    fn new<'a>(bar: MultiProgress) -> Box<dyn ProgressStarter<'a> + 'a> {
        Box::new(IndicatifProgressStarter { bar })
    }
}


#[async_trait]
impl<'a> ProgressStarter<'a> for IndicatifProgressStarter {
    async fn start(&'a self, start_message: String, range: ProgressRange) -> Box<dyn ProgressNotifier + 'a> {
        let progress = self.bar.add(new_progress_bar(start_message.as_ref(), range));
        let self_observer: Arc<&'a dyn Observer> = Arc::new(self);
        let observer = Arc::downgrade(&self_observer);
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

struct CurrentProgress<'a> {
    observer: Weak<&'a dyn Observer>,
    bar: ProgressBar,
}

impl<'a> Drop for CurrentProgress<'a> {
    fn drop(&mut self) {
        self.observer.upgrade().map(|o| o.notify(&(self.bar)));
    }
}

impl<'a> CurrentProgress<'a> {
    fn new(observer: Weak<&'a dyn Observer>, bar: ProgressBar) -> Box<dyn ProgressNotifier + 'a> {
        Box::new(
            CurrentProgress {
                observer,
                bar,
            }
        )
    }
}

#[async_trait]
impl<'a> ProgressNotifier for CurrentProgress<'a> {
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
