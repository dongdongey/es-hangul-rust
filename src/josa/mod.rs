// crate/josa/mod.rs

mod josa; //josa.rs
use self::josa::josa;
pub use self::josa::JosaOption;

impl crate::Josa for str {
    fn josa(&self, josa_option: josa::JosaOption) -> String {
        crate::josa::josa(&self, josa_option)
    }
}
