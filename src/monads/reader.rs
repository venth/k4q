use std::marker::PhantomData;

pub struct Reader<'reader, CTX, T> {
    f: Box<dyn 'reader + Fn(&CTX) -> T>,
    context_type: PhantomData<CTX>,
}

impl<'reader, CTX: 'reader, T: 'reader> Reader<'reader, CTX, T>
{
    pub fn new<F: 'reader + Fn(&CTX) -> T>(f: F) -> Self {
        Self { f: Box::new(f), context_type: PhantomData }
    }

    pub fn unit(t: T) -> Self
        where
            T: 'reader + Clone,
    {
        Self::new(move |_| t.clone())
    }

    pub fn and_then<V: 'reader, G: 'reader>(self, f: G) -> Reader<'reader, CTX, V>
        where
            G: Fn(T) -> Reader<'reader, CTX, V>,
    {
        Reader::new(move |ctx: &CTX| f((self.f)(&ctx)).apply(&ctx))
    }

    pub fn map<V: 'reader, G: 'reader>(self, f: G) -> Reader<'reader, CTX, V>
        where
            G: Fn(T) -> V,
    {
        Reader::<CTX, V>::new(move |ctx: &CTX| f((self.f)(&ctx)))
    }

    pub fn apply(&self, ctx: &CTX) -> T {
        (self.f)(ctx)
    }
}

#[cfg(test)]
mod tests {
    use super::Reader;

    struct Dao {
        age: u16,
    }

    fn user_by_age<'a>(age: u16) -> Reader<'a, Dao, u16> {
        Reader::<'a, Dao, u16>::new(move |dao| dao.age + age)
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
}
