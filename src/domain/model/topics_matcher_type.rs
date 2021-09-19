pub enum TopicsMatcherType<'a> {
    DIRECT(Vec<&'a str>),
    REGEX(Vec<&'a str>),
}
