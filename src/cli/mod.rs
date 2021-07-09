use crate::domain::query::Action;

mod cli_configuration;
mod query_key_equlas_value;

pub fn parse<'a>() -> Option<impl Action + 'a> {
    let matched = cli_configuration::app()
        .get_matches();

    query_key_equlas_value::matches(&matched)
}
