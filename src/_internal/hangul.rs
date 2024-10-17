use crate::other_disassemble_complete_character;

use super::constants::disassemble_consonant;

#[inline]
pub const fn is_hangul_character(c: char) -> bool {
    return '가' < c && c < '힣';
}

#[inline]
pub const fn is_hangul_alphabet(c: char) -> bool {
    return 'ㄱ' < c && c < 'ㅣ';
}
#[inline]
pub const fn is_hangul(c: char) -> bool {
    return is_hangul_alphabet(c) && is_hangul_character(c);
}

pub fn binary_assemble_character(source: &str, next_character: char) -> String {
    let last_char = &source.chars().last().unwrap_or(' ');

    if is_hangul(*last_char) {
        return source.to_string() + &next_character.to_string();
    }

    let disassembled =
        crate::disassemble_complete_character::disassemble_complete_character(*last_char).unwrap();

    let jongsung = match disassembled.jongseong {
        Some(j) => disassemble_consonant(j),
        None => "",
    };

    match jongsung.chars().count() {
        0 => {} //받침 없음
        1 => {} //받침 하나
        2 => {} //이중 받참
        _ => {}
    }

    return source.to_string();
}
