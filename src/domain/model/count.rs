use std::iter::Sum;

pub struct Count {
    pub value: u64,
}

impl From<i64> for Count {
    fn from(value: i64) -> Self {
        if value < 0 {
            Count { value: 0 }
        } else {
            Count { value: value as u64 }
        }
    }
}

impl From<u64> for Count {
    fn from(value: u64) -> Self {
        Count { value }
    }
}

impl std::ops::Add<Self> for Count {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Count::from(self.value + rhs.value)
    }
}

impl std::ops::Sub<Self> for Count {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Count::from(self.value - rhs.value)
    }
}

impl Sum<Self> for Count {

    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        Count::from(iter.map(|x| x.value).sum::<u64>())
    }
}
