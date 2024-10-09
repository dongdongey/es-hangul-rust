pub mod has_batchim;

pub use has_batchim::{has_batchim, KindOfBatchim};

impl crate::HasBatchim for &'static str {
    fn has_batchim(self, option: KindOfBatchim) -> bool {
        has_batchim(self, option)
    }
}

impl crate::HasBatchim for String {
    fn has_batchim(self, option: KindOfBatchim) -> bool {
        has_batchim(self.as_str(), option)
    }
}
