pub mod get_choseong;
pub use get_choseong::get_choseong;

use crate::GetChoseong;

impl GetChoseong for str {
    fn get_choseong(&self) -> String {
        get_choseong(self)
    }
}

#[test]
fn test() {
    assert_eq!("오 마이 갓!".get_choseong(), "ㅇ ㅁㅇ ㄱ!");
    let a = "지금 이 순간...".to_string();
    assert_eq!(a.get_choseong(), "ㅈㄱ ㅇ ㅅㄱ...");
}
