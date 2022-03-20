use std::iter;

pub trait IntoIteratedResultEx<'a, I, E>: Sized
    where I: Iterator, I::Item: 'a, E: 'a
{
    fn into_iterated_result(self) -> Box<dyn Iterator<Item=Result<I::Item, E>> + 'a>;
}

impl<'a, T: 'a, I: 'a + Iterator<Item=T>, E: 'a> IntoIteratedResultEx<'a, I, E> for Result<I, E> {
    fn into_iterated_result(self) -> Box<dyn Iterator<Item=Result<I::Item, E>> + 'a> {
        fn error_as_iterator<'a, T: 'a, E: 'a>(err: E) -> Box<dyn Iterator<Item=Result<T, E>> + 'a>{
            Box::new(iter::once(Result::<T, E>::Err(err)))
        }

        self
            .map(|it| it.map(Result::<I::Item, E>::Ok))
            .map(|it| Box::new(it) as Box<dyn Iterator<Item=Result<I::Item, E>>>)
            .unwrap_or_else(error_as_iterator)
    }
}

#[cfg(test)]
mod tests {
    use crate::iter::into_iterated_result::IntoIteratedResultEx;

    #[test]
    fn converts_error_into_iterator_with_error() {
        // given
        let error_message = "an error";
        let error = Result::<Box<dyn Iterator<Item=&str>>, &str>::Err(error_message);

        // when
        let mut iter = error.into_iterated_result();

        // then
        assert_eq!(error_message, iter.next().unwrap().err().unwrap());
        assert_eq!(false, iter.next().is_some());
    }

    #[test]
    fn converts_success_iterator_result_into_iterator_of_success() {
        // given
        let sequence = [1, 2, 3];
        let success = Result::<Box<dyn Iterator<Item=&i32>>, &str>::Ok(Box::new(sequence.iter()));

        // when
        let mut iter = success.into_iterated_result();

        // then
        assert_eq!(&sequence[0], iter.next().unwrap().ok().unwrap());
        assert_eq!(&sequence[1], iter.next().unwrap().ok().unwrap());
        assert_eq!(&sequence[2], iter.next().unwrap().ok().unwrap());
        assert_eq!(false, iter.next().is_some());
    }
}
