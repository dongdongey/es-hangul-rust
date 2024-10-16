use crate::{_internal::constants, disassemble_complete_character::disassemble_complete_character};

pub fn disassemble_to_groups(string: &str) -> Vec<Vec<char>> {
    string
        .chars()
        .map(|c| -> Vec<char> {
            match disassemble_complete_character(c) {
                Some(gulja) => {
                    let mut ret_vec: Vec<char> = Vec::new();

                    ret_vec.push(gulja.choseong);

                    for disassembled in constants::disassemble_vowel(gulja.jungseong).chars() {
                        ret_vec.push(disassembled);
                    }

                    if let Some(jong) = gulja.jongseong {
                        for disassembled in constants::disassemble_consonant(jong).chars() {
                            ret_vec.push(disassembled);
                        }
                    }

                    return ret_vec;
                }
                None => return vec![c],
            }
        })
        .collect()
}

pub fn disassemble(string: &str) -> String {
    disassemble_to_groups(string)
        .into_iter()
        .flat_map(|v_c| v_c.into_iter())
        .collect::<String>()
}

#[test]
fn test() {
    assert_eq!(
        disassemble_to_groups("아싸 나는 없다."),
        [
            vec!['ㅇ', 'ㅏ'],
            vec!['ㅆ', 'ㅏ'],
            vec![' '],
            vec!['ㄴ', 'ㅏ'],
            vec!['ㄴ', 'ㅡ', 'ㄴ'],
            vec![' '],
            vec!['ㅇ', 'ㅓ', 'ㅂ', 'ㅅ'],
            vec!['ㄷ', 'ㅏ'],
            vec!['.']
        ]
    );

    assert_eq!(
        disassemble("아싸 나는 없다."),
        "ㅇㅏㅆㅏ ㄴㅏㄴㅡㄴ ㅇㅓㅂㅅㄷㅏ."
    );
}
