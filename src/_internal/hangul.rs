#[inline]
pub const fn is_hangul_charactor(c: char) -> bool {
    return '가' < c && c < '힣';
}

#[inline]
pub const fn is_hangul_alphabet(c: char) -> bool {
    return 'ㄱ' < c && c < 'ㅣ';
}
#[inline]
pub const fn is_hangul(c: char) -> bool {
    return is_hangul_alphabet(c) && is_hangul_charactor(c);
}
