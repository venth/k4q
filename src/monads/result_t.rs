use std::sync::Arc;

use do_notation::Lift;

use crate::monads::Reader;

pub struct ResultT<'a, CTX, S, E> {
    value: Arc<Reader<'a, CTX, Result<S, E>>>,
}

impl<'a, CTX: 'a, S: 'a, E: 'a> ResultT<'a, CTX, S, E> {
    pub fn lift(value: Reader<'a, CTX, Result<S, E>>) -> Self {
        Self { value: Arc::new(value) }
    }

    pub fn value(&self) -> &Reader<'a, CTX, Result<S, E>> {
        &(self.value)
    }

    pub fn map<V, F>(&self, f: F) -> ResultT<'a, CTX, V, E>
        where
            V: 'a,
            F: 'a + Fn(S) -> V,
    {
        ResultT::lift(self.value.map(move |r| r.map(|s| f(s))))
    }

    pub fn map_err<V, F>(&self, f: F) -> ResultT<'a, CTX, S, V>
        where
            V: 'a,
            F: 'a + Fn(E) -> V,
    {
        ResultT::lift(self.value.map(move |r| r.map_err(|s| f(s))))
    }

    pub fn and_then<V, F>(&self, f: F) -> ResultT<'a, CTX, V, E>
        where
            V: 'a,
            F: 'a + Fn(S) -> ResultT<'a, CTX, V, E>,
    {
        let func = Arc::new(f);
        let reader = self.value.clone();
        let mapped = Reader::new(move |ctx| {
            reader.map(|r| {
                let func1 = func.clone();
                r.and_then(move |s| (func1.clone())(s).value.apply(ctx))
            }).apply(ctx)
        });
        ResultT::lift(mapped)

        /*        ResultT::lift(self.value
                    .and_then(move |v| {
                        Reader::new(move |ctx| {
                            Self::duplicate(&v)
                                .and_then(|s| f(s).value.apply(ctx))
                        })
                    }))
        */
    }
}

fn duplicate<S, E>(v: &Result<S, E>) -> Result<S, E>
    where S: Clone, E: Clone,
{
    match &v {
        Ok(s) => Ok(Clone::clone(s)),
        Err(e) => Err(Clone::clone(e))
    }
}


impl<'reader, CTX: 'reader, S: 'reader + Clone, E: 'reader + Clone> Lift<Result<S, E>> for ResultT<'reader, CTX, S, E> {
    fn lift(a: Result<S, E>) -> Self {
        ResultT::lift(Reader::new(move |_| duplicate(&a)))
    }
}

#[cfg(test)]
mod test {
    use do_notation::m;

    use super::Reader;
    use super::ResultT;

    #[test]
    fn transforms_reader_with_result_into_result_transformer() {
        // given
        let original = Reader::<String, Result<String, ()>>::unit(Result::Ok("OK".to_string()));

        // and
        let transformed = ResultT::lift(original);

        // when
        let message = transformed.value().apply(&"some".to_string());

        // then
        assert_eq!("OK", message.unwrap());
    }

    #[test]
    fn maps_success_value_info_a_new_one() {
        // given
        let original = Reader::<String, Result<String, ()>>::unit(Result::Ok("OK".to_string()));

        // and
        let transformed = ResultT::lift(original);

        // and
        let mapped = transformed.map(move |s| format!("mapped {}", s));

        // when
        let message = mapped.value().apply(&"some".to_string());

        // then
        assert_eq!("mapped OK", message.unwrap());
    }

    #[test]
    fn maps_success_to_an_error() {
        // given
        let original = Reader::<String, Result<String, String>>::unit(Result::Ok("OK".to_string()));

        // and
        let transformed = ResultT::lift(original);

        // and
        let mapped: ResultT<String, String, String> = transformed.and_then(move |s|
            ResultT::lift(Reader::unit(Result::Err(format!("error instead of {}", s)))));

        // when
        let message = mapped.value().apply(&"some".to_string());

        // then
        assert_eq!("error instead of OK", message.err().unwrap());
    }

    #[test]
    fn maps_success_to_different_type() {
        // given
        let original = Reader::<String, Result<String, String>>::unit(Result::Ok("OK".to_string()));

        // and
        let transformed = ResultT::lift(original);

        // and
        let mapped: ResultT<String, i32, String> = transformed.and_then(move |_|
            ResultT::lift(Reader::unit(Result::Ok(1))));

        // when
        let value = mapped.value().apply(&"some".to_string());

        // then
        assert_eq!(1, value.unwrap());
    }

    #[test]
    fn maps_error_to_a_different_error_kind() {
        // given
        let original = Reader::<String, Result<String, ()>>::unit(Result::Err(()));

        // and
        let transformed = ResultT::lift(original);

        // and
        let mapped: ResultT<String, String, String> = transformed.map_err(move |_| "error".to_string());

        // when
        let message = mapped.value().apply(&"some".to_string());

        // then
        assert_eq!("error", message.err().unwrap());
    }

    #[test]
    fn enables_do_notation() {
        // given
        let read_msg = Reader::<i32, Result<String, ()>>::unit(Result::Ok("OK".to_string()));
        let result_msg = ResultT::lift(read_msg);
        // when
        let res = m! {
            msg <- result_msg;
            let do_msg = format!("do_{}", msg);
            return Result::Ok(do_msg);
        };

        // then
        let some_context = 1;
        assert_eq!("do_OK", res.value().apply(&some_context).unwrap())
    }
}
