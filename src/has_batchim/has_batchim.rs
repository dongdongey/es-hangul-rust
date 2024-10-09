use crate::_internal::constants::{self, get_disassembled_jongseong};

pub enum KindOfBatchim {
    Single,
    Double,
    None,
}

pub fn has_batchim(guljas: &str, option: KindOfBatchim) -> bool {
    let last_char = match guljas.chars().last() {
        Some(a) => a,
        None => {
            return false;
        }
    };

    let char_code = last_char as usize;

    let is_complete_hangul = constants::COMPLETE_HANGUL_START_CHARCODE <= char_code
        && char_code <= constants::COMPLETE_HANGUL_END_CHARCODE;

    if !is_complete_hangul {
        return false;
    }

    let jongseong_index =
        (char_code - constants::COMPLETE_HANGUL_START_CHARCODE) % constants::NUMBER_OF_JONGSEONG;

    match option {
        KindOfBatchim::Single => {
            if let Some(disassembled) =
                get_disassembled_jongseong(constants::JONGSEONGS[jongseong_index])
            {
                return disassembled.chars().count() == 1;
            }
            return false;
        }
        KindOfBatchim::Double => {
            if let Some(disassembled) =
                get_disassembled_jongseong(constants::JONGSEONGS[jongseong_index])
            {
                return disassembled.chars().count() == 2;
            }
            return false;
        }
        KindOfBatchim::None => {
            return jongseong_index > 0;
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::has_batchim::has_batchim::has_batchim;
    use crate::has_batchim::has_batchim::KindOfBatchim;
    #[test]
    fn batchim___() {
        assert_eq!(has_batchim("앗", KindOfBatchim::None), true);
        assert_eq!(has_batchim("흙", KindOfBatchim::Double), true);
        assert_eq!(has_batchim("흙", KindOfBatchim::Single), false);
        assert_eq!(has_batchim("어", KindOfBatchim::None), false);
    }
}
