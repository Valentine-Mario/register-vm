use crate::assembler::Token;
use crate::assembler::opcode_parser::*;
use nom::types::CompleteStr;
use crate::assembler::operand_parser::operand;
use crate::assembler::label_parser::label_declaration;
use crate::assembler::directive_parser::directive;
//use the assembler instruction struct to combine different parser from opcode parser
#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    pub opcode: Option<Token>,
    pub operand1: Option<Token>,
    pub operand2: Option<Token>,
    pub operand3: Option<Token>,
    pub label: Option<Token>,
    pub directive: Option<Token>
}


named!(instruction_combined<CompleteStr, AssemblerInstruction>,
    do_parse!(
        l: opt!(label_declaration) >>
        o: opcode >>
        o1: opt!(operand) >>
        o2: opt!(operand) >>
        o3: opt!(operand) >>
        (
            AssemblerInstruction{
                opcode: Some(o),
                label: l,
                directive: None,
                operand1: o1,
                operand2: o2,
                operand3: o3,
            }
        )
    )
);

// Will try to parse out any of the Instruction forms
named!(pub instruction<CompleteStr, AssemblerInstruction>,
    do_parse!(
        ins: alt!(
            instruction_combined | directive
        ) >>
        (
            ins
        )
    )
);

impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results:Vec<u8> = vec![];
        if let Some(ref token) = self.opcode{
            match token {
                Token::Op { code } => match code {
                    _ => {
                        let b: u8 = (*code).into();
                        results.push(b);

                    }
                },
                _ => {
                    println!("Non-opcode found in opcode field");
                    std::process::exit(1);
                }
            };
    
            for operand in &[&self.operand1, &self.operand2, &self.operand3] {
                if let Some(token) = operand {
                    AssemblerInstruction::extract_operand(token, &mut results)
                }
             }
             while results.len() < 4 {
                results.push(0);
            }
        }

        return results;
    }
    pub fn is_label(&self) -> bool {
        self.label.is_some()
    }

    pub fn get_label_name(&self) -> Option<String> {
        match &self.label {
            Some(l) => match l {
                Token::LabelDeclaration { name } => Some(name.clone()),
                _ => None,
            },
            None => None,
        }
    }


    //convert numbers to bytecode
    fn extract_operand(t: &Token, results: &mut Vec<u8>) {
        match t {
            Token::Register { reg_num } => {
                results.push(*reg_num);
            }
            Token::IntegerOperand { value } => {
                let converted = *value as u16;
                let byte1 = converted;
                let byte2 = converted >> 8;
                results.push(byte2 as u8);
                results.push(byte1 as u8);
            }
            _ => {
                println!("Opcode found in operand field");
                std::process::exit(1);
            }
        };
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::assembler::Token;
    use crate::instruction::Opcode;
    
    #[test]
    fn test_parse_instruction_form_one() {
        let result = instruction_combined(CompleteStr("load $0 #100\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Some(Token::Op { code: Opcode::LOAD }),
                    label: None,
                    directive: None,
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::IntegerOperand { value: 100 }),
                    operand3: None
                }
            ))
        );
    }

    
    #[test]
    fn test_parse_instruction_form_two() {
        let result = instruction_combined(CompleteStr("hlt"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Some(Token::Op { code: Opcode::HLT }),
                    label: None,
                    directive: None,
                    operand1: None,
                    operand2: None,
                    operand3: None
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction_form_three() {
        let result = instruction_combined(CompleteStr("add $0 $1 $2\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Some(Token::Op { code: Opcode::ADD }),
                    label: None,
                    directive: None,
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::Register { reg_num: 1 }),
                    operand3: Some(Token::Register { reg_num: 2 }),
                }
            ))
        );
    }


}