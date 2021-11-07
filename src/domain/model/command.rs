use crate::domain::model::criteria::Criteria;
use crate::domain::model::topics_matcher_type::TopicsMatcherType;

pub enum Command {
    QueryByKey(TopicsMatcherType, Box<dyn Criteria>),
    CommandNotRecognized,
}
