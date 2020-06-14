use crate::instruction::Opcode;
pub mod opcode_parser;
pub mod instruction_parsers;
pub mod program_parser;
// use crate::assembler::opcode::opcode_parsers;
// use crate::opcode::operand_parsers;
// use crate::opcode::register_parsers;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op{code: Opcode},
    Register{reg_num: u8},
    IntegerOperand{value: i32},
}
