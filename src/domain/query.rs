pub trait Action {
    fn execute<'a>(&self) -> Box<dyn Iterator<Item=Record> + 'a>;
}

pub trait Criteria {
    fn test(&self, rec: &Record) -> bool;
}

#[derive(Debug)]
pub struct Record {}

#[derive(Debug)]
pub struct KeyValue {
    value: String,
}

impl From<&str> for KeyValue {
    fn from(value: &str) -> Self {
        Self { value: String::from(value) }
    }
}

pub fn key_equals_value(value: KeyValue) -> impl Criteria {
    CriteriaKeyEqualsValue { key: value }
}

pub fn using<'a>(criteria: impl Criteria + 'static, topics: Vec<&str>) -> impl Action + 'a {
    QueryAction {
        criteria: Box::from(criteria),
        topics: topics.iter().map(|e| String::from(*e)).collect(),
    }
}

struct QueryAction {
    criteria: Box<dyn Criteria>,
    topics: Vec<String>,
}

impl Action for QueryAction {
    fn execute<'a>(&self) -> Box<dyn Iterator<Item=Record> + 'a> {
        let _ = self.criteria.test(&Record {});
        Box::new(std::iter::empty::<Record>())
    }
}

impl Action for Box<dyn Action> {
    fn execute<'a>(&self) -> Box<dyn Iterator<Item=Record> + 'a> {
        self.as_ref().execute()
    }
}


impl Criteria for Box<dyn Criteria> {
    fn test(&self, rec: &Record) -> bool {
        self.as_ref().test(&rec)
    }
}

impl Criteria for CriteriaKeyEqualsValue {
    fn test(&self, rec: &Record) -> bool {
        false
    }
}

#[derive(Debug)]
struct CriteriaKeyEqualsValue {
    key: KeyValue,
}
