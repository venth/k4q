use std::ops::Sub;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Watermark {
    value: i64,
}

impl From<i64> for Watermark {
    fn from(value: i64) -> Self {
        Watermark { value }
    }
}

impl Sub for Watermark{
    type Output = i64;

    fn sub(self, rhs: Self) -> Self::Output {
        self.value - rhs.value
    }
}

impl Sub for &Watermark{
    type Output = i64;

    fn sub(self, rhs: Self) -> Self::Output {
        self.value - rhs.value
    }
}
