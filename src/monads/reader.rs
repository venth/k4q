use std::marker::PhantomData;
use std::sync::Arc;

use do_notation::Lift;

pub struct Reader<'reader, CTX, T> {
    f: Arc<dyn 'reader + Fn(&CTX) -> T>,
    context_type: PhantomData<CTX>,
}

impl<'reader, CTX: 'reader, T: 'reader> Reader<'reader, CTX, T>
{
    pub fn new<F: 'reader + Fn(&CTX) -> T>(f: F) -> Self {
        Self { f: Arc::new(f), context_type: PhantomData }
    }

    pub fn unit(t: T) -> Self
        where T: Clone,
    {
        Self::new(move |_| t.clone())
    }

    pub fn and_then<V: 'reader, G: 'reader>(self, f: G) -> Reader<'reader, CTX, V>
        where
            G: 'reader + Fn(T) -> Reader<'reader, CTX, V>,
    {
        Reader::<'reader, CTX, V>::new(move |ctx: &CTX| f((self.f.clone())(&ctx)).apply(&ctx))
    }

    pub fn map<V: 'reader, G: 'reader>(self, f: G) -> Reader<'reader, CTX, V>
        where
            G: Fn(T) -> V,
    {
        let func = self.f.clone();
        Reader::<'reader, CTX, V>::new(move |ctx: &CTX| f(func(&ctx)))
    }

    pub fn apply(&self, ctx: &CTX) -> T {
        (self.f.clone())(ctx)
    }
}

impl<'reader, CTX: 'reader, A: 'reader + Clone> Lift<A> for Reader<'reader, CTX, A> {
    fn lift(a: A) -> Self {
        Reader::unit(a)
    }
}

#[cfg(test)]
mod tests {
    use do_notation::m;

    use super::Reader;

    struct Dao {
        age: u16,
    }

    #[test]
    fn lifts_function_into_reader() {
        // given
        let user = Reader::new(move |dao: &Dao| dao.age + 15);

        // and
        let dao = Dao { age: 15 };

        // when
        let result = user.apply(&dao);

        // then
        assert_eq!(dao.age + 15, result);
    }

    #[test]
    fn lifts_value_into_reader() {
        // given
        let value = 16;
        let user = Reader::unit(value);

        // and
        let dao = Dao { age: 15 };

        // when
        let result = user.apply(&dao);

        // then
        assert_eq!(value, result);
    }

    #[test]
    fn maps_value_contained_in_reader() {
        // given
        let value = 16;
        let user = Reader::unit(value);

        // and
        let dao = Dao { age: 15 };

        // and
        let mapped_user = user.map(move |v| v + 1);

        // when
        let result = mapped_user.apply(&dao);

        // then
        assert_eq!(value + 1, result);
    }

    #[test]
    fn flat_maps_value_contained_in_reader() {
        // given
        let value = 16;
        let user = Reader::new(|dao: &Dao| dao.age + value);

        // and
        let dao = Dao { age: 15 };

        // and
        macro_rules! user_details_by {($age: expr, $value: expr) => (format!("age: {:?}, v: {:?}", $age, $value))}

        let user_details = user
            .and_then(move |v|
                Reader::new(move |dao: &Dao| user_details_by!(dao.age, v)));

        // when
        let result = user_details.apply(&dao);

        // then
        assert_eq!(user_details_by!(dao.age, dao.age + value), result);
    }

    #[test]
    fn enables_do_notation() {
        // given
        let read_msg = Reader::<i32, String>::unit("OK".to_string());

        // when
        let res = m! {
            msg <- read_msg;
            let do_msg = format!("do_{}", msg);
            return do_msg;
        };

        // then
        let some_context = 1;
        assert_eq!("do_OK", res.apply(&some_context))
    }
}
