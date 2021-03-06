use crate::instruction;
const PIE_HEADER_PREFIX: [u8; 4] = [45, 50, 49, 45];
const PIE_HEADER_LENGTH: usize = 64;

pub struct VM{
   /// Array that simulates having hardware registers
    pub registers: [i32; 32],
    /// Program counter that tracks which byte is being executed
    pc: usize,
    /// The bytecode of the program being run
    pub program: Vec<u8>,
    /// Contains the remainder of modulo division ops
    remainder: usize,
    /// Contains the result of the last comparison operation
    equal_flag: bool,
    //contains the head for our vm memory
    heap:Vec<u8>
}

impl VM{
    pub fn new()->VM{
        VM{
            registers:[0;32],
            pc:0,
            program:vec![],
            remainder:0,
            equal_flag:false,
            heap:vec![]
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

    pub fn add_byte(&mut self, b: u8) {
        self.program.push(b);
    }

    // Adds an arbitrary byte to the VM's program
    pub fn add_bytes(&mut self, mut b: Vec<u8>) {
        self.program.append(&mut b);
    }


    pub fn clear_program(&mut self){
        self.program=vec![];
    }

    /// Processes the header of bytecode the VM wants to execute
    pub fn verify_header(&self) -> bool {
        if self.program[0..4] != PIE_HEADER_PREFIX {
            return false;
        }
        true
    }

   
    
    fn execute_instruction(&mut self)->bool{
         // If our program counter has exceeded the length of the program itself, something has
            // gone awry
            if self.pc>=self.program.len(){
                return true
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
                self.remainder = (register1 % register2) as usize;
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
            //check equality and store them in the equal flag
            instruction::Opcode::EQ=>{
                let register1= self.registers[self.next_8_bits() as usize];
                let register2=self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 == register2;
                self.next_8_bits();
            },
            //check if not equal
            instruction::Opcode::NEQ=>{
                let register1= self.registers[self.next_8_bits() as usize];
                let register2=self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 != register2;
                self.next_8_bits();
            },
            //check for greater than
            instruction::Opcode::GT=>{
                let register1=self.registers[self.next_8_bits() as usize];
                let register2=self.registers[self.next_8_bits() as usize];
                self.equal_flag= register1>register2;
                self.next_8_bits();
            },
            //check less than
            instruction::Opcode::LT=>{
                let register1=self.registers[self.next_8_bits() as usize];
                let register2=self.registers[self.next_8_bits() as usize];
                self.equal_flag=register1<register2;
                self.next_8_bits();
            },
            //greater than or equal to
            instruction::Opcode::GTQ=>{
                let register1=self.registers[self.next_8_bits() as usize];
                let register2=self.registers[self.next_8_bits() as usize];
                self.equal_flag=register1>=register2;
                self.next_8_bits();
            },
            //less than or equal to
            instruction::Opcode::LTQ=>{
                let register1=self.registers[self.next_8_bits() as usize];
                let register2=self.registers[self.next_8_bits() as usize];
                self.equal_flag=register1<=register2;
                self.next_8_bits();
            }
            //ump If Equal will take one register as an argument, and if equal_flag is true, will jump to the value stored in that register
            instruction::Opcode::JEQ=>{
                let register = self.next_8_bits() as usize;
                let target = self.registers[register];
                if self.equal_flag {
                    self.pc = target as usize;
                }
            },
            //relative jump backward
            instruction::Opcode::JMPB=>{
                let value=self.registers[self.next_8_bits()as usize] as usize;
                self.pc+=value;
            },
            //aloc extends the size of the heap vector by the amount in the register given as an argument.
            instruction::Opcode::ALOC=>{
                let register = self.next_8_bits() as usize;
                let bytes = self.registers[register];
                let new_end = self.heap.len() as i32 + bytes;
                self.heap.resize(new_end as usize, 0);
            },
            
                instruction::Opcode::HLT=>{
                    println!("HLT encountered");
                    return true;
                },  
                _=>{
                    println!("unrecognized instruction");
                    return true;
                }  
            }
            false
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
 pub fn prepend_header(mut b: Vec<u8>) -> Vec<u8> {
        let mut prepension = vec![];
        for byte in PIE_HEADER_PREFIX.iter() {
            prepension.push(byte.clone());
        }
        while prepension.len() <= PIE_HEADER_LENGTH {
            prepension.push(0);
        }
        prepension.append(&mut b);
        prepension
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
fn test_eq_opcode() {
    let mut test_vm = VM::new();
    test_vm.registers[0] = 10;
    test_vm.registers[1] = 10;
    test_vm.program = vec![9, 0, 1, 0, 9, 0, 1, 0];
    test_vm.run_once();
    assert_eq!(test_vm.equal_flag, true);
    test_vm.registers[1] = 20;
    test_vm.run_once();
    assert_eq!(test_vm.equal_flag, false);
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

    #[test]
    //greter than or equal
    fn test_gte_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 20;
        test_vm.registers[1] = 10;
        test_vm.program = vec![11, 0, 1, 0, 11, 0, 1, 0, 11, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[0] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[0] = 5;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_neq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 20;
        test_vm.program = vec![10, 0, 1, 0, 10, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_lte_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 20;
        test_vm.registers[1] = 10;
        test_vm.program = vec![12, 0, 1, 0, 12, 0, 1, 0, 12, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[0] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[0] = 5;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_lt_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 20;
        test_vm.registers[1] = 10;
        test_vm.program = vec![13, 0, 1, 0, 13, 0, 1, 0, 13, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[0] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[0] = 5;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_gt_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 20;
        test_vm.registers[1] = 10;
        test_vm.program = vec![14, 0, 1, 0, 14, 0, 1, 0, 14, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[0] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[0] = 5;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_jeq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 7;
        test_vm.equal_flag = true;
        test_vm.program = vec![15, 0, 0, 0, 15, 0, 0, 0, 15, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 7);
    }
    #[test]
fn test_aloc_opcode() {
    let mut test_vm = VM::new();
    test_vm.registers[0] = 500;
    test_vm.program = vec![17, 0, 0, 0];
    test_vm.run_once();
    assert_eq!(test_vm.heap.len(), 500);
}

}