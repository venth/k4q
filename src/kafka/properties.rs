use serde;

#[derive(Debug, serde::Deserialize)]
pub struct KafkaProperties {
    pub bootstrap: BootstrapProperties,
    pub group: Group,

}

#[derive(Debug, serde::Deserialize)]
pub struct BootstrapProperties {
    pub servers: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Group {
    pub id: String,
}
