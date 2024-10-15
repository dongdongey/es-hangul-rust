pub mod has_batchim;

pub use has_batchim::{has_batchim, KindOfBatchim};

impl crate::HasBatchim for str {
    fn has_batchim(&self, option: KindOfBatchim) -> bool {
        let last_char = match self.chars().last() {
            Some(a) => a,
            None => {
                return false;
            }
        };

        has_batchim(last_char, option)
    }
}
impl crate::HasBatchim for char {
    fn has_batchim(&self, option: KindOfBatchim) -> bool {
        has_batchim(*self, option)
    }
}
