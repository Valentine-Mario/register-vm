use nom::types::CompleteStr;

#[derive(Debug, PartialEq)]
pub enum Opcode{
    HLT,//halt
    IGL,
    LOAD,//load program
    ADD,//add
    SUB,
    MUL,
    DIV,
    JMP,//jump
    JMPF,//jump forward
    JMPB,//jump backward
    EQ,//equal
    NEQ,//not equal
    GT,//greter than
    LT,//less than
    GTQ,//greater than or equal to
    LTQ, //less than or equal to
    JEQ,//jump if equal
    NOP,
    ALOC //for allocating memory to the heap

}

#[derive(Debug, PartialEq)]
pub struct Instruction{
    opcode:Opcode
}

impl Instruction{
    pub fn new(opcode:Opcode)->Instruction{
        Instruction{
            opcode:opcode
        }
    }
}
impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => Opcode::LOAD,
            1 => Opcode::ADD,
            2 => Opcode::SUB,
            3 => Opcode::MUL,
            4 => Opcode::DIV,
            5 => Opcode::HLT,
            6 => Opcode::JMP,
            7=> Opcode::JMPF,
            8=> Opcode::JMPB,
            9=>Opcode::EQ,
            10=>Opcode::NEQ,
            11=>Opcode::GTQ,
            12=>Opcode::LTQ,
            13=>Opcode::LT,
            14=>Opcode::GT,
            15=>Opcode::JEQ,
            16=>Opcode::NOP,
            17=>Opcode::ALOC,
            _ => Opcode::IGL
        }
    }
}


impl<'a> From<CompleteStr<'a>> for Opcode {
    fn from(v: CompleteStr<'a>) -> Self {
        match v {
            CompleteStr("load") => Opcode::LOAD,
            CompleteStr("add") => Opcode::ADD,
            CompleteStr("sub") => Opcode::SUB,
            CompleteStr("mul") => Opcode::MUL,
            CompleteStr("div") => Opcode::DIV,
            CompleteStr("hlt") => Opcode::HLT,
            CompleteStr("jmp") => Opcode::JMP,
            CompleteStr("jmpf") => Opcode::JMPF,
            CompleteStr("jmpb") => Opcode::JMPB,
            CompleteStr("eq") => Opcode::EQ,
            CompleteStr("neq") => Opcode::NEQ,
            CompleteStr("gte") => Opcode::GTQ,
            CompleteStr("gt") => Opcode::GT,
            CompleteStr("lte") => Opcode::LTQ,
            CompleteStr("lt") => Opcode::LT,
            CompleteStr("jmpe") => Opcode::JEQ,
            CompleteStr("nop") => Opcode::NOP,
            CompleteStr("aloc")=>Opcode::ALOC,
            _ => Opcode::IGL,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
      let instruction = Instruction::new(Opcode::HLT);
      assert_eq!(instruction.opcode, Opcode::HLT);
    }
}