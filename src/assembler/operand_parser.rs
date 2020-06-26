use crate::assembler::register_parser::register;
use crate::assembler::Token;
use nom::digit;
use nom::types::CompleteStr;
use crate::assembler::opcode_parser::integer_operand;


named!(pub operand<CompleteStr, Token>,
    alt!(
        integer_operand |
        register
    )
);
