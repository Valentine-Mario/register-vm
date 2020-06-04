use crate::instruction;
pub struct VM{
    registers:[i32; 32],//array of i32 to hold 32 bits
    pc:usize,//program counter
    program:Vec<u8>,//store our program bytecode
    remainder: u32,//for division
}

impl VM{
    pub fn new()->VM{
        VM{
            registers:[0;32],
            pc:0,
            program:vec![],
            remainder:0
        }
    }

    pub fn run(&mut self){
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self){
        self.execute_instruction();
    }
    
    fn execute_instruction(&mut self)->bool{
         // If our program counter has exceeded the length of the program itself, something has
            // gone awry
            if self.pc>=self.program.len(){
                return false
            }
            match self.decode_opcode(){
                //1. Decode the first 8 bits and see LOAD 2. Decode the next 8 bits and use it to get the register 3. Decode the next 16 bits (split into 2 u8s) into an integer 4. Store them in the register
                instruction::Opcode::LOAD=>{
                    let register = self.next_8_bits() as usize; // We cast to usize so we can use it as an index into the array
                    let number = self.next_16_bits() as u16;
                    self.registers[register] = number as i32; // Our registers are i32s, so we need to cast it. We'll cover that later.
                },
                //LOAD $0 #10 LOAD $1 #15ADD $0 $1 $2
                instruction::Opcode::ADD=>{
                    let register1=self.registers[self.next_8_bits() as usize];
                    let register2=self.registers[self.next_8_bits() as usize];
                    self.registers[self.next_8_bits() as usize]= register1+register2;
                },
                instruction::Opcode::SUB=>{
                    let register1=self.registers[self.next_8_bits() as usize];
                    let register2=self.registers[self.next_8_bits() as usize];
                    self.registers[self.next_8_bits() as usize]=register1-register2;
                },
                instruction::Opcode::MUL=>{
                    let register1=self.registers[self.next_8_bits() as usize];
                    let register2=self.registers[self.next_8_bits() as usize];
                    self.registers[self.next_8_bits() as usize]=register1*register2;
                },
                //When we come across a DIV opcode, what we want to do is divide it, store the quotient in the register, and the remainder in the remainder attribute of the VM
               instruction::Opcode::DIV=>{
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
               },
               //jump to an instruction in the program
               instruction::Opcode::JMP=>{
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
               },
               //relative jump forward
               instruction::Opcode::JMPF => {
                let value = self.registers[self.next_8_bits() as usize] as usize;
                self.pc += value;
            },
            //relative jump backward
            instruction::Opcode::JMPB=>{
                let value=self.registers[self.next_8_bits()as usize] as usize;
                self.pc+=value;
            }
                instruction::Opcode::HLT=>{
                    println!("HLT encountered");
                    false;
                },  
                _=>{
                    println!("unrecognized instruction");
                }  
            }
            true
    }
    fn decode_opcode(&mut self) -> instruction::Opcode {
        //get cureent opcode and move to next byte
        let opcode = instruction::Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }
    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        return result;
    }
    
    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        return result;
    }
   

}

#[cfg(test)]
mod tests{
    use super::*;

#[test]
fn test_vm(){
    let vm=VM::new();
    assert_eq!(vm.registers[0], 0)
}
#[test]
fn test_jmpf_opcode() {
    let mut test_vm = VM::new();
    test_vm.registers[0] = 2;
    test_vm.program = vec![8, 0, 0, 0, 6, 0, 0, 0];
    test_vm.run_once();
    assert_eq!(test_vm.pc, 4);
}
#[test]
    fn test_jmpb_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[1] = 6;
        test_vm.program = vec![0, 0, 0, 10, 8, 1, 0, 0];//little endian representation for opcode 9
        test_vm.run_once();
       test_vm.run_once();
        assert_eq!(test_vm.pc, 12);
    }

#[test]
fn test_load_opcode() {
  let mut test_vm = VM::new();
  test_vm.program = vec![0, 0, 1, 244]; // Remember, this is how we represent 500 using two u8s in little endian format
  test_vm.run();
  assert_eq!(test_vm.registers[0], 500);
}


#[test]
fn test_jmp_opcode() {
    let mut test_vm = VM::new();
    test_vm.registers[0] = 1;
    test_vm.program = vec![7, 0, 0, 0];
    test_vm.run_once();
    assert_eq!(test_vm.pc, 3);
}
    #[test]
    fn test_opcode_hlt() {
      let mut test_vm = VM::new();
      let test_bytes = vec![0,0,0,0]; //little endian representation
      test_vm.program = test_bytes;
      test_vm.run();
      assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_opcode_igl() {
      let mut test_vm = VM::new();
      let test_bytes = vec![200,0,0,0];
      test_vm.program = test_bytes;
      test_vm.run();
      assert_eq!(test_vm.pc, 1);
    }
}