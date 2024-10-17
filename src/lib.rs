mod _internal;
pub mod combine_character;
pub mod disassemble;
pub use disassemble::disassemble::{disassemble, disassemble_to_groups};
pub trait Disassemble {
    fn disassemble(&self) -> String;
    fn disassemble_to_groups(&self) -> Vec<Vec<char>>;
}

pub mod disassemble_complete_character;
pub use disassemble_complete_character::disassemble_complete_character as other_disassemble_complete_character;

pub mod has_batchim;
pub use has_batchim::{has_batchim as other_has_batchim, KindOfBatchim};
//HasBatchim trait for declarative programming
//implementations are in has_batchim module
pub trait HasBatchim {
    fn has_batchim(&self, option: KindOfBatchim) -> bool;
}
//

pub mod josa;
//Josa trait for declarative programming
//implementations are in josa module
pub trait Josa {
    fn josa(&self, josa_option: josa::JosaOption) -> String;
}

pub mod get_choseong_mod;
pub use get_choseong_mod::get_choseong;
pub trait GetChoseong {
    fn get_choseong(&self) -> String;
}
