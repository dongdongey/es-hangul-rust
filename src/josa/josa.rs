use crate::disassemble_complete_character;
use crate::has_batchim;

#[derive(PartialEq)]
pub enum JosaOption {
    이_가,
    을_를,
    은_는,
    으로_로,
    와_과,
    이나_나,
    이란_란,
    아_야,
    이랑_랑,
    이에요_예요,
    으로서_로서,
    으로써_로써,
    으로부터_로부터,
    이라_라,
}

impl JosaOption {
    const fn to_strs(&self) -> (&'static str, &'static str) {
        match self {
            JosaOption::이_가 => ("이", "가"),
            JosaOption::을_를 => ("을", "를"),
            JosaOption::은_는 => ("은", "는"),
            JosaOption::으로_로 => ("으로", "로"),
            JosaOption::와_과 => ("와", "과"),
            JosaOption::이나_나 => ("이나", "나"),
            JosaOption::이란_란 => ("이란", "란"),
            JosaOption::아_야 => ("아", "야"),
            JosaOption::이랑_랑 => ("이랑", "랑"),
            JosaOption::이에요_예요 => ("이에요", "예요"),
            JosaOption::으로서_로서 => ("으로서", "로서"),
            JosaOption::으로써_로써 => ("으로써", "로써"),
            JosaOption::으로부터_로부터 => ("으로부터", "로부터"),
            JosaOption::이라_라 => ("이라", "라"),
        }
    }
}

const 로_조사: &[JosaOption] = &[
    JosaOption::으로_로,
    JosaOption::으로서_로서,
    JosaOption::으로써_로써,
    JosaOption::으로부터_로부터,
];

pub fn josa(word: String, josa: JosaOption) -> String {
    if word.len() == 0 {
        return word;
    }

    return word.clone() + josa_picker(word.as_str(), josa);
}

fn josa_picker(word: &str, josa: JosaOption) -> &str {
    if word.chars().count() == 0 {
        return josa.to_strs().0;
    }

    let has받침: bool = has_batchim::has_batchim(word, None);
    let mut index: bool = if has받침 { false } else { true };

    let is_종성_리을: bool = disassemble_complete_character::disassemble_complete_character(
        word.chars().last().unwrap(),
    )
    .unwrap()
    .jongseong
        == "ㄹ";

    let is_case_of_로 = has받침 && is_종성_리을 && 로_조사.contains(&josa);

    if josa == JosaOption::와_과 || is_case_of_로 {
        index = !index;
    }

    let is_end_with_이 = word.chars().last() == Some('이');

    if josa == JosaOption::이에요_예요 && is_end_with_이 {
        index = true;
    }

    let ret = if index {
        josa.to_strs().1
    } else {
        josa.to_strs().0
    };

    return ret;
}

#[cfg(test)]
mod tests {
    use crate::josa::josa::{josa, JosaOption};

    #[test]
    fn josa_nani() {
        assert_eq!(
            josa("동동게이".to_string(), JosaOption::은_는),
            "동동게이는".to_string()
        );
    }
}
