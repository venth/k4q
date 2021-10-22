#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(into="String")]
pub struct RecordKey {
    value: String,
}

impl From<RecordKey> for String {
    fn from(t: RecordKey) -> Self {
        t.value
    }
}

impl From<&str> for RecordKey {
    fn from(value: &str) -> Self {
        Self { value: String::from(value) }
    }
}

impl Clone for RecordKey {
    fn clone(&self) -> Self {
        Self { value: self.value.clone() }
    }
}
