use std;
use std::io;
use std::io::Write;
//import the vm
use crate::vm::VM;
use nom::types::CompleteStr;
use crate::assembler::program_parser::program;

//core structure of the repl for the assembler
pub struct REPL{
    command_buffer:Vec<String>,//vec of executed programs
    vm:VM
}

impl REPL{
    pub fn new()->REPL{
        REPL{
            vm:VM::new(),
            command_buffer:vec![]
        }
    }

    pub fn run(&mut self) {
        println!("Welcome to my vm! Let's be productive!");
        loop {
            // This allocates a new String in which to store whatever the user types each iteration.
            // TODO: Figure out how create this outside of the loop and re-use it every iteration
            let mut buffer = String::new();
    
            // Blocking call until the user types in a command
            let stdin = io::stdin();
    
            // Annoyingly, `print!` does not automatically flush stdout like `println!` does, so we
            // have to do that there for the user to see our `>>> ` prompt.
            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");
    
            // Here we'll look at the string the user gave us.
            stdin.read_line(&mut buffer).expect("Unable to read line from user");
            let buffer = buffer.trim();

            // This is the line we add to store a copy of each command history
            self.command_buffer.push(buffer.to_string());
            match buffer {
                ".quit" => {
                    println!("Farewell! Have a great day!");
                    std::process::exit(0);
                },
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                },
                ".registers"=>{
                    println!("listing all programs in memory");
                    println!("{:?}", self.vm.program);
                },
                _ => {
                    let parsed_program = program(CompleteStr(buffer));
                if !parsed_program.is_ok() {
                    println!("Unable to parse input");
                    continue;
                }
                let (_, result) = parsed_program.unwrap();
                let bytecode = result.to_bytes();
                println!("{:?}", bytecode);
                // TODO: Make a function to let us add bytes to the VM
                for byte in bytecode {
                    self.vm.add_byte(byte);
                }
                self.vm.run_once();
                }
            }
        }
    }
}