use crate::_internal::{
    constants::{self},
    hangul::is_hangul_charactor,
};

pub fn get_choseong(string: &str) -> String {
    let a: String = string
        .chars()
        .map(|c| -> char {
            if !is_hangul_charactor(c) {
                return c;
            } else {
                let hangul_code: usize = c as usize - constants::COMPLETE_HANGUL_START_CHARCODE;

                let choseong_index: usize =
                    (hangul_code / constants::NUMBER_OF_JONGSEONG) / constants::NUMBER_OF_JUNGSEONG;

                return constants::CHOSEONGS[choseong_index];
            }
        })
        .collect();

    return a;
}

#[test]
fn test() {
    assert_eq!(get_choseong("이 망할 fuck"), "ㅇ ㅁㅎ fuck".to_string())
}
