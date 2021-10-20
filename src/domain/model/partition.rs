#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Partition {
    value: i32,
}

impl From<&i32> for Partition {
    fn from(value: &i32) -> Self {
        Self { value: *value }
    }
}
