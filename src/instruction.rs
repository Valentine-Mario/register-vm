#[derive(Debug, PartialEq)]
pub enum Opcode{
    HLT,
    IGL,
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    JMPF,
    JMPB
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