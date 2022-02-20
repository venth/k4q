use core::option::Option;
use std::{iter, thread};

use crossbeam::channel;
use crossbeam::channel::{Receiver, Sender};
use rayon::iter::ParallelIterator;

pub trait IntoSequentialIteratorEx<'a, T: Sized>: Sized {
    fn into_seq_iter(self) -> Box<dyn 'a + Iterator<Item=T>>;
}

impl<'a, T, PI> IntoSequentialIteratorEx<'a, T> for PI
    where
        T: 'a + Send,
        PI: 'a + ParallelIterator<Item=T>,
{
    fn into_seq_iter(self) -> Box<dyn 'a + Iterator<Item=T>> {
        let (sender, receiver) = channel::unbounded();

        Box::new(deferred_first_element(self, sender, receiver.clone())
            .chain(deferred_remaining_elements(receiver)))
    }
}

fn deferred_first_element<'a, T: 'a + Send, PI: 'a + ParallelIterator<Item=T>>(
    par_iter: PI,
    sender: Sender<T>,
    receiver: Receiver<T>) -> Box<dyn 'a + Iterator<Item=T>>
{
    let deferred = iter::once(Box::new(move || {

        crossbeam::scope(|s| {
            s.spawn(|_| {
                par_iter.for_each(|element| {
                    sender.send(element).unwrap();
                });
            });
        }).unwrap();

        receiver.recv().ok()
    }) as Box<dyn FnOnce() -> Option<T>>);

    Box::new(deferred
        .map(|f| f())
        .filter(Option::is_some)
        .map(Option::unwrap))
}

fn deferred_remaining_elements<'a, T: 'a + Send>(receiver: Receiver<T>) -> Box<dyn 'a + Iterator<Item=T>> {
    Box::new(
        iter::repeat_with(move || receiver.recv().ok())
            .filter(Option::is_some)
            .map(Option::unwrap))
}
