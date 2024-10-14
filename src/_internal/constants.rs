pub const COMPLETE_HANGUL_START_CHARCODE: usize = '가' as usize;
pub const COMPLETE_HANGUL_END_CHARCODE: usize = '힣' as usize;

pub const NUMBER_OF_JONGSEONG: usize = 28;
pub const NUMBER_OF_JUNGSEONG: usize = 21;

pub const fn disassemble_consonant(consonant: char) -> &'static str {
    match consonant {
        'ㄱ' => "ㄱ",
        'ㄲ' => "ㄲ",
        'ㄳ' => "ㄱㅅ",
        'ㄴ' => "ㄴ",
        'ㄵ' => "ㄴㅈ",
        'ㄶ' => "ㄴㅎ",
        'ㄷ' => "ㄷ",
        'ㄸ' => "ㄸ",
        'ㄹ' => "ㄹ",
        'ㄺ' => "ㄹㄱ",
        'ㄻ' => "ㄹㅁ",
        'ㄼ' => "ㄹㅂ",
        'ㄽ' => "ㄹㅅ",
        'ㄾ' => "ㄹㅌ",
        'ㄿ' => "ㄹㅍ",
        'ㅀ' => "ㄹㅎ",
        'ㅁ' => "ㅁ",
        'ㅂ' => "ㅂ",
        'ㅃ' => "ㅃ",
        'ㅄ' => "ㅂㅅ",
        'ㅅ' => "ㅅ",
        'ㅆ' => "ㅆ",
        'ㅇ' => "ㅇ",
        'ㅈ' => "ㅈ",
        'ㅉ' => "ㅉ",
        'ㅊ' => "ㅊ",
        'ㅋ' => "ㅋ",
        'ㅌ' => "ㅌ",
        'ㅍ' => "ㅍ",
        'ㅎ' => "ㅎ",
        _ => "", // 종성이 없는 경우 빈 문자열
    }
}

pub const fn disassemble_vowel(vowel: char) -> &'static str {
    match vowel {
        'ㅏ' => "ㅏ",
        'ㅐ' => "ㅐ",
        'ㅑ' => "ㅑ",
        'ㅒ' => "ㅒ",
        'ㅓ' => "ㅓ",
        'ㅔ' => "ㅔ",
        'ㅕ' => "ㅕ",
        'ㅖ' => "ㅖ",
        'ㅗ' => "ㅗ",
        'ㅘ' => "ㅗㅏ",
        'ㅙ' => "ㅗㅐ",
        'ㅚ' => "ㅗㅣ",
        'ㅛ' => "ㅛ",
        'ㅜ' => "ㅜ",
        'ㅝ' => "ㅜㅓ",
        'ㅞ' => "ㅜㅔ",
        'ㅟ' => "ㅜㅣ",
        'ㅠ' => "ㅠ",
        'ㅡ' => "ㅡ",
        'ㅢ' => "ㅡㅣ",
        'ㅣ' => "ㅣ",
        _ => "", // 빈 문자열 처리
    }
}

pub static CHOSEONGS: &[char] = &[
    'ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㅊ', 'ㅋ',
    'ㅌ', 'ㅍ', 'ㅎ',
];

pub fn is_choseong(c: char) -> bool {
    CHOSEONGS.contains(&c)
}

pub static JUNSEONGS: &[char] = &[
    'ㅏ', 'ㅐ', 'ㅑ', 'ㅒ', 'ㅓ', 'ㅔ', 'ㅕ', 'ㅖ', 'ㅗ', 'ㅘ', 'ㅙ', 'ㅚ', 'ㅛ', 'ㅜ', 'ㅝ', 'ㅞ',
    'ㅟ', 'ㅠ', 'ㅡ', 'ㅢ', 'ㅣ',
];

pub fn is_junseong(vowel: char) -> bool {
    JUNSEONGS.contains(&vowel)
}

pub static JONGSEONGS: &[Option<char>] = &[
    None,
    Some('ㄱ'),
    Some('ㄲ'),
    Some('ㄳ'),
    Some('ㄴ'),
    Some('ㄵ'),
    Some('ㄶ'),
    Some('ㄷ'),
    Some('ㄹ'),
    Some('ㄺ'),
    Some('ㄻ'),
    Some('ㄼ'),
    Some('ㄽ'),
    Some('ㄾ'),
    Some('ㄿ'),
    Some('ㅀ'),
    Some('ㅁ'),
    Some('ㅂ'),
    Some('ㅄ'),
    Some('ㅅ'),
    Some('ㅆ'),
    Some('ㅇ'),
    Some('ㅈ'),
    Some('ㅊ'),
    Some('ㅋ'),
    Some('ㅌ'),
    Some('ㅍ'),
    Some('ㅎ'),
];

#[inline]
pub fn get_disassembled_jongseong(jongseong: Option<char>) -> Option<&'static str> {
    match jongseong {
        Some(c) => {
            if JONGSEONGS.contains(&jongseong) {
                return Some(disassemble_consonant(c));
            } else {
                return None;
            }
        }
        None => None,
    }
}
