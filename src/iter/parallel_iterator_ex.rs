use rayon::iter::{Map, ParallelIterator};

pub trait ParallelIteratorEx<T: Send, E: Send> {
    fn map_ok<'a, F, U>(self, f: F) -> Map<Self, Box<dyn Fn(Result<T, E>) -> Result<U, E> + Sync + Send + 'a>>
        where U: Send,
              F: 'a + Fn(T) -> U + Sync + Send,
              Self: ParallelIterator<Item=Result<T, E>>,;
}

impl<T: Send, E: Send, I: ParallelIterator<Item=Result<T, E>>> ParallelIteratorEx<T, E> for I {
    fn map_ok<'a, F, U>(self, f: F) -> Map<Self, Box<dyn Fn(Result<T, E>) -> Result<U, E> + Sync + Send + 'a>>
        where U: Send, F: 'a + Fn(T) -> U + Sync + Send, Self: ParallelIterator<Item=Result<T, E>>
    {
        fn success_using<'a, T, U, E, F>(f: F) -> Box<dyn Fn(Result<T, E>) -> Result<U, E> + Sync + Send + 'a>
            where T: Send, U: Send, E: Send,
                  F: 'a + Fn(T) -> U + Sync + Send,
        {
            Box::new(move |r: Result<T, E>| r.map(|s| f(s)))
        }

        self.map(success_using(f))
    }
}
