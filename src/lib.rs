mod _internal;
pub mod disassemble;
pub mod disassemble_complete_character;

pub mod has_batchim;
pub use has_batchim::{has_batchim as other_has_batchim, KindOfBatchim};
//HasBatchim trait for declarative programming
//implementations are in has_batchim module
pub trait HasBatchim {
    fn has_batchim(self, option: KindOfBatchim) -> bool;
}
//

pub mod josa;
//Josa trait for declarative programming
//implementations are in josa module
pub trait Josa {
    fn josa(self, josa_option: josa::JosaOption) -> String;
}
