pub struct Reader<'a, CTX, T> {
    f: Box<dyn Fn(&'a CTX) -> T + 'a>,
}

impl<'a, CTX, T> Reader<'a, CTX, T> {
    pub fn new<F>(f: F) -> Self
        where
            F: 'a + Fn(&'a CTX) -> T,
    {
        Self { f: Box::new(f) }
    }

    pub fn of(t: T) -> Self
        where
            T: 'a + Clone
    {
        Self { f: Box::new(move |_| t.clone()) }
    }

    pub fn flat_map<G, V>(self, f: G) -> Reader<'a, CTX, V>
        where
            T: 'a,
            G: 'a + Fn(T) -> Reader<'a, CTX, V>,
    {
        Reader::new(move |ctx| f((self.f)(ctx)).apply(ctx))
    }

        pub fn map<V, F>(self, f: F) -> Reader<'a, CTX, V>
            where
                T: 'a,
                F: 'a + Fn(T) -> V,
        {
            Reader::new(move |ctx| f((self.f)(ctx)))
        }

    pub fn apply(&self, ctx: &'a CTX) -> T {
        (self.f)(ctx)
    }
}
