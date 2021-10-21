use crate::domain::model::PartitionId;
use crate::domain::model::watermark::Watermark;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Partition {
    pub partition_id: PartitionId,
    pub low_watermark: Watermark,
    pub high_watermark: Watermark,
}

impl Partition {
    pub fn new(partition_id: PartitionId, low_watermark: Watermark, high_watermark: Watermark)
               -> Self {
        Partition {
            partition_id,
            low_watermark,
            high_watermark
        }
    }

    pub fn record_count(&self) -> i64 {
        &self.high_watermark - &self.low_watermark
    }
}
