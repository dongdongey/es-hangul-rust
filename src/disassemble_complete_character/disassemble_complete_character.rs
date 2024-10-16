use crate::_internal::constants;

pub struct DisassembledCompleteCharacter {
    pub choseong: char,
    pub jungseong: char,
    pub jongseong: Option<char>,
}

// 완성된 한글 문자를 분해하는 함수
pub fn disassemble_complete_character(letter: char) -> Option<DisassembledCompleteCharacter> {
    let char_code: usize = letter as usize;

    // 주어진 문자가 완성된 한글인지 확인
    if !(constants::COMPLETE_HANGUL_START_CHARCODE <= char_code
        && char_code <= constants::COMPLETE_HANGUL_END_CHARCODE)
    {
        return None;
    }

    let hangul_code: usize = char_code - constants::COMPLETE_HANGUL_START_CHARCODE;

    let jongseong_index: usize = hangul_code % constants::NUMBER_OF_JONGSEONG;
    let jungseong_index: usize =
        (hangul_code / constants::NUMBER_OF_JONGSEONG) % constants::NUMBER_OF_JUNGSEONG;
    let choseong_index: usize =
        (hangul_code / constants::NUMBER_OF_JONGSEONG) / constants::NUMBER_OF_JUNGSEONG;

    Some(DisassembledCompleteCharacter {
        choseong: constants::CHOSEONGS[choseong_index],
        jungseong: constants::JUNSEONGS[jungseong_index],
        jongseong: constants::JONGSEONGS[jongseong_index],
    })
}

#[cfg(test)]
mod tests {
    use crate::disassemble_complete_character::disassemble_complete_character::{
        self, disassemble_complete_character,
    };

    #[test]
    fn jamoeum_bunly() {
        let g_a_g = disassemble_complete_character::disassemble_complete_character('각').unwrap();
        assert_eq!(g_a_g.choseong, 'ㄱ');
        assert_eq!(g_a_g.jungseong, 'ㅏ');
        assert_eq!(g_a_g.jongseong, Some('ㄱ'));

        let a = disassemble_complete_character('아').unwrap();
        assert_eq!(a.choseong, 'ㅇ');
        assert_eq!(a.jungseong, 'ㅏ');
        assert_eq!(a.jongseong, None);
    }
}
