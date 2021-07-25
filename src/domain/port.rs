use crate::domain::action::Action;

pub trait RecordFinder<'a> {
    
}

pub trait ActionRecognizer<'a> {
    fn recognize<'b>(&self, args: &'b Vec<&'b str>) -> Option<Box<dyn Action<'a> +'a>>;
}
