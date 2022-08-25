use serde;

#[derive(Copy, Debug, serde::Serialize, serde::Deserialize)]
#[serde(into="String")]
pub struct PartitionId {
    value: i32,
}

unsafe impl Send for PartitionId {}

impl From<PartitionId> for String {
    fn from(t: PartitionId) -> Self {
        t.value.to_string()
    }
}

impl From<i32> for PartitionId {
    fn from(value: i32) -> Self {
        Self { value }
    }
}

impl Clone for PartitionId {
    fn clone(&self) -> Self {
        Self { value: self.value.clone() }
    }
}

impl PartitionId {
    pub fn value(&self) -> i32 {
        self.value
    }
}