#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PartitionId {
    value: i32,
}

impl From<i32> for PartitionId {
    fn from(value: i32) -> Self {
        Self { value }
    }
}
