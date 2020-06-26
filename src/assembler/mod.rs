use crate::instruction::Opcode;
pub mod opcode_parser;
pub mod instruction_parsers;
pub mod program_parser;
pub mod register_parser;
pub mod operand_parser;
pub mod label_parser;
pub mod directive_parser;
// use crate::assembler::opcode::opcode_parsers;
// use crate::opcode::operand_parsers;
// use crate::opcode::register_parsers;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op{code: Opcode},
    Register{reg_num: u8},
    IntegerOperand{value: i32},
    LabelDeclaration { name: String },
    LabelUsage { name: String },
    Directive { name: String }
}
