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
    ALOC, //for allocating memory to the heap
    INC,
    DEC

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
            18=>Opcode::INC,
            19=>Opcode::DEC,
            _ => Opcode::IGL
        }
    }
}

impl From<Opcode> for u8 {
    fn from(op: Opcode) -> Self {
        match op {
            Opcode::LOAD => 0,
            Opcode::ADD => 1,
            Opcode::SUB => 2,
            Opcode::MUL => 3,
            Opcode::DIV => 4,
            Opcode::HLT => 5,
            Opcode::JMP => 6,
            Opcode::JMPF => 7,
            Opcode::JMPB => 8,
            Opcode::EQ => 9,
            Opcode::NEQ => 10,
            Opcode::GTQ => 11,
            Opcode::LTQ => 12,
            Opcode::LT => 13,
            Opcode::GT => 14,
            Opcode::JEQ => 15,
            Opcode::NOP => 16,
            Opcode::ALOC => 17,
            Opcode::INC => 18,
            Opcode::DEC => 19,
            Opcode::IGL=>20
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
            CompleteStr("inc")=>Opcode::INC,
            CompleteStr("dec")=>Opcode::DEC,
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