pub mod vm;
pub mod instruction;
pub mod repl;
pub mod assembler;

#[macro_use]
extern crate nom;

#[macro_use]
extern crate clap;

use clap::{Arg, App, SubCommand};
use std::{path::Path, fs::File, io::Read};


fn main() {

    // let mut repl = repl::REPL::new();
    // repl.run();

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let target_file = matches.value_of("INPUT_FILE");
    match target_file {
        Some(filename) => {
            let program = read_file(filename);
            let mut asm = assembler::Assembler::new();
            let mut vm = vm::VM::new();
            let program = asm.assemble(&program);
            match program {
                Some(p) => {
                    vm.add_bytes(p);
                    vm.run();
                    std::process::exit(0);
                },
                None => {}
            }
        },
        None => {
            start_repl();
        }
    }
}
//start a repl that would run
fn start_repl() {
    let mut repl = repl::REPL::new();
    repl.run();
}

//read file
fn read_file(tmp: &str) -> String {
    let filename = Path::new(tmp);
    match File::open(Path::new(&filename)) {
      Ok(mut fh) => {
        let mut contents = String::new();
        match fh.read_to_string(&mut contents) {
          Ok(_) => {
            return contents;
          },
          Err(e) => {
            println!("There was an error reading file: {:?}", e);
            std::process::exit(1);
          }
        }
      },
      Err(e) => {
        println!("File not found: {:?}", e);
        std::process::exit(1)
      }
    }
}