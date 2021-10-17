pub trait Progress {
    fn message(&self, msg: &str);
    fn increase(&self);
    fn finish(&self);
}
