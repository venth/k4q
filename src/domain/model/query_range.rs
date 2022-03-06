pub enum QueryRange {
    Whole,
}

unsafe impl Send for QueryRange {}
unsafe impl Sync for QueryRange {}
