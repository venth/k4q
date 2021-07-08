use crate::domain::query::Action;

mod cli;
mod domain;

fn main() {
    let action = cli::parse();
    action.unwrap()
        .execute();
    /*
        println!("topics: {}, criteria: {}, key value: {}",
                 query_topics, query_key_criteria, query_key_value)
    */
}
