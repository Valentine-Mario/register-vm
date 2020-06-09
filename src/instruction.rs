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
            _ => Opcode::IGL
        }
    }
}
// impl From<Opcode> for u8 {
//     fn from(op: Opcode) -> Self {
//         match op {
//             Opcode::LOAD => 0,
//             Opcode::ADD => 1,
//             Opcode::SUB => 2,
//             Opcode::MUL => 3,
//             Opcode::DIV => 4,
//             Opcode::HLT => 5,
//             Opcode::JMP => 6,
//             Opcode::IGL=>7,
//         }
//     }
// }

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