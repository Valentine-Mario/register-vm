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
  

// create a function named register that accepts a CompleteStr and returns a CompleteStr and Token or an Error
named!(pub register <CompleteStr, Token>, 
    //use the ws! macro, which tells it to consume any whitespace on either side of our register. This lets us write variants such as LOAD $0 in addition to LOAD $0
    ws!(
        //use the do_parse! macro to chain parsers
        do_parse!( 
            //use tag! to look for $, pass the result of tag!…
            tag!("$") >> 
            // function digit, and save the result in a variable called reg_num. nom provides the function digit, which recognizes one or more 0-9 characters
            reg_num: digit >>
            (
                //Create the Token enum with the appropriate info and return
                Token::Register{ 
                  reg_num: reg_num.parse::<u8>().unwrap()
                } 
            ) 
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
    fn test_parse_register() {
        let result = register(CompleteStr("$0"));
        assert_eq!(result.is_ok(), true);
        let (rest, num)=result.unwrap();
        assert_eq!(rest, CompleteStr(""));
        assert_eq!(num, Token::Register{reg_num: 0});
        let result = register(CompleteStr("0"));
        assert_eq!(result.is_ok(), false);
        let result = register(CompleteStr("$a"));
        assert_eq!(result.is_ok(), false);
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