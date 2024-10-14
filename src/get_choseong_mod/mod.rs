pub mod get_choseong;
pub use get_choseong::get_choseong;

use crate::GetChoseong;

impl GetChoseong for &str {
    fn get_choseong(self) -> String {
        get_choseong(self.to_string())
    }
}
impl GetChoseong for String {
    fn get_choseong(self) -> String {
        get_choseong(self)
    }
}
