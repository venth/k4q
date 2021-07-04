mod cli;


fn main() {
    let matches = cli::app();

    let query_topics = matches
        .subcommand_matches("query")
        .and_then(|m| m.values_of("topics"))
        .map(|v| v.collect())
        .unwrap_or(vec!["No topics"])
        .join(", ");

    let query_key_criteria = matches
        .subcommand_matches("query")
        .and_then(|m| m.subcommand_matches("key"))
        .and_then(|m| m.value_of("criteria"))
        .unwrap_or("no criteria");

    let query_key_value = matches
        .subcommand_matches("query")
        .and_then(|m| m.subcommand_matches("key"))
        .and_then(|m| m.value_of("keyValue"))
        .unwrap_or("no value");

    println!("topics: {}, criteria: {}, key value: {}",
             query_topics, query_key_criteria, query_key_value)
}
