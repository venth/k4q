#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(into="String")]
pub struct Offset {
    value: i64,
}

impl From<Offset> for String {
    fn from(t: Offset) -> Self {
        t.value.to_string()
    }
}

impl From<i64> for Offset {
    fn from(value: i64) -> Self {
        Self { value }
    }
}

impl Clone for Offset {
    fn clone(&self) -> Self {
        Self { value: self.value.clone() }
    }
}
