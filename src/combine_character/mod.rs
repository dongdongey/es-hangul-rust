use crate::_internal::constants;

pub fn combine_character(choseong: char, jungseong: char, jongseong: Option<char>) -> Option<char> {
    if !(constants::is_choseong(choseong) && constants::is_junseong(jungseong)) {
        return None;
    }

    if jongseong.is_some() && !constants::is_jongseong(jongseong.unwrap()) {
        return None;
    }
    let cho_code = choseong as usize - 'ㄱ' as usize;
    let jung_code = jungseong as usize - 'ㅏ' as usize;
    let jong_code = jongseong.map_or(0, |j| j as usize - 'ㄱ' as usize - 1);

    let choseong_of_target_consonant =
        cho_code * constants::NUMBER_OF_JONGSEONG * constants::NUMBER_OF_JUNGSEONG;

    let choseong_of_target_vowel = jung_code * constants::NUMBER_OF_JONGSEONG;

    let code = constants::COMPLETE_HANGUL_START_CHARCODE
        + choseong_of_target_consonant
        + choseong_of_target_vowel
        + jong_code;

    return char::from_u32(code as u32);
}

#[test]
fn test() {
    assert_eq!(combine_character('ㄱ', 'ㅏ', None), Some('가'));
    assert_eq!(combine_character('ㄱ', 'ㅏ', Some('ㅇ')), Some('강'));
    assert_eq!(combine_character('ㅄ', 'ㅏ', Some('ㅇ')), None);
}
