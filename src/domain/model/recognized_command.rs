#[cfg_attr(test, derive(Clone))]
#[derive(Debug)]
pub(crate) enum RecognizedCommand {
    Unrecognized(String),
    QueryByKey(RecognizedTopicPattern, RecognizedQueryTerm),
}


#[cfg_attr(test, derive(Clone))]
#[derive(Debug)]
pub(crate) enum RecognizedQueryTerm {
    Key(RecognizedCriteria),
}

#[cfg_attr(test, derive(Clone))]
#[derive(Debug)]
pub(crate) enum RecognizedCriteria {
    Eq(String),
}

#[cfg_attr(test, derive(Clone))]
#[derive(Debug)]
pub(crate) enum RecognizedTopicPattern {
    Direct(Vec<String>),
}
