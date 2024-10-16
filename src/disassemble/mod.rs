pub mod disassemble;

impl crate::Disassemble for str {
    fn disassemble(&self) -> String {
        disassemble::disassemble(self)
    }
    fn disassemble_to_groups(&self) -> Vec<Vec<char>> {
        disassemble::disassemble_to_groups(self)
    }
}
