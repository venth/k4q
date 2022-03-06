pub trait IntoCompleteAwareIteratorEx<'a, I: Iterator>: Sized {
    fn into_complete_aware_iter(self, on_completed: impl Fn() -> () + Send + 'a)
                                -> IntoCompleteAwareIterator<'a, I::Item>;
}

impl<'a, I: 'a + Iterator + Send> IntoCompleteAwareIteratorEx<'a, I> for I {
    fn into_complete_aware_iter(self, on_completed: impl Fn() -> () + Send + 'a)
                                -> IntoCompleteAwareIterator<'a, I::Item> {
        IntoCompleteAwareIterator::<I::Item> {
            decorated: Box::new(self),
            on_completed: Box::new(on_completed),
        }
    }
}

pub struct IntoCompleteAwareIterator<'a, T> {
    decorated: Box<dyn Iterator<Item=T> + Send + 'a>,
    on_completed: Box<dyn Fn() -> () + Send + 'a>,
}

impl<'a, T: Send> Iterator for IntoCompleteAwareIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let el = self.decorated.next();
        if el.is_some() {
            el
        } else {
            (self.on_completed)();
            Option::None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::iter;
    use std::sync::Arc;

    use mockall::automock;

    use crate::iter::finish_aware_iterator::IntoCompleteAwareIteratorEx;

    // cannot mock function, therefore trait Callback is introduced with
    // the static function
    #[automock]
    trait Callback {
        fn on_completed(&self) -> ();
    }

    #[test]
    fn executes_on_completed_for_empty_iterator() {
        // given
        let mut mocked_callback = MockCallback::new();
        mocked_callback.expect_on_completed().times(1).returning(|| ());
        // and
        let mut empty_iterator = iter::empty::<i32>()
            .into_complete_aware_iter(move || mocked_callback.on_completed());

        // when
        assert_eq!(Option::None, empty_iterator.next());
    }

    #[test]
    fn executes_on_completed_when_iterator_is_exhausted() {
        // given
        let sequence_size = 10;
        // and
        let mut mocked_callback = MockCallback::new();
        mocked_callback.expect_on_completed().times(1).returning(|| ());
        // and
        let mut iterator = iter::repeat(1).take(sequence_size)
            .into_complete_aware_iter(|| mocked_callback.on_completed());

        // when
        (0..sequence_size).for_each(|_| {
            iterator.next();
        });

        // then
        assert_eq!(Option::None, iterator.next());
    }

    #[test]
    fn does_not_execute_on_completed_when_iterator_still_has_elements() {
        // given
        let sequence_size = 10;
        // and
        let mut mocked_callback = MockCallback::new();
        // and
        let mut iterator = iter::repeat(1).take(sequence_size)
            .into_complete_aware_iter(|| mocked_callback.on_completed());

        // when
        (0..sequence_size - 1).for_each(|_| {
            iterator.next();
        });

        // then
        assert_eq!(true, iterator.next().is_some());
    }

    #[test]
    fn does_call_on_completed_only_once() {
        // given
        let sequence_size = 10;
        // and
        let mut mocked_callback = MockCallback::new();
        mocked_callback.expect_on_completed().times(0).returning(|| ());
        // and
        let mut iterator = iter::repeat(1).take(sequence_size)
            .into_complete_aware_iter(|| mocked_callback.on_completed());

        // when
        (0..sequence_size - 1).for_each(|_| {
            iterator.next();
        });

        // then
        assert_eq!(true, iterator.next().is_some());
    }
}
