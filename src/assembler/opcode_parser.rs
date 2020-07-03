use nom::types::CompleteStr;
use nom::digit;
use nom::alpha1;
use crate::assembler::Token;
use crate::instruction;
use crate::assembler::register_parser;
//recognize opcode string from instructions
named!(pub opcode<CompleteStr, Token>,
    do_parse!(
        //alpha1 recognizes one or more lowercase and uppercase alphabetic characters For ASCII strings: a-zA-Z For UTF8 strings, any alphabetic code point 
        opcode: alpha1 >>
        (
          {
              Token::Op{code: instruction::Opcode::from(opcode)}
          }
        )
    )
  );
  


named!(pub integer_operand<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("#") >>
            reg_num: digit >>
            (
                Token::IntegerOperand{value: reg_num.parse::<i32>().unwrap()}
            )
        )
    )
);


mod tests {
    use super::*;

    #[test]
    fn test_opcode_load() {
        // First tests that the opcode is detected and parsed correctly
        let result = opcode(CompleteStr("load"));
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op{code: instruction::Opcode::LOAD});
        assert_eq!(rest, CompleteStr(""));

        // Tests that an invalid opcode isn't recognized got igl
        let result = opcode(CompleteStr("aold"));
        assert_eq!(result.is_ok(), true);
    }

    

    #[test]
fn test_parse_integer_operand() {
    // Test a valid integer operand
    let result = integer_operand(CompleteStr("#10"));
    assert_eq!(result.is_ok(), true);
    let (rest, value) = result.unwrap();
    assert_eq!(rest, CompleteStr(""));
    assert_eq!(value, Token::IntegerOperand{value: 10});

    // Test an invalid one (missing the #)
    let result = integer_operand(CompleteStr("10"));
    assert_eq!(result.is_ok(), false);
}

#[test]
fn test_opcode() {
    let result = opcode(CompleteStr("load"));
    assert_eq!(result.is_ok(), true);
    let (rest, token) = result.unwrap();
    assert_eq!(token, Token::Op { code: instruction::Opcode::LOAD });
    assert_eq!(rest, CompleteStr(""));
    let result = opcode(CompleteStr("aold"));
    let (_, token) = result.unwrap();
    assert_eq!(token, Token::Op { code: instruction::Opcode::IGL });
    let result = opcode(CompleteStr("div"));
    let (_, token) = result.unwrap();
    assert_eq!(token, Token::Op { code: instruction::Opcode::DIV });
}
}