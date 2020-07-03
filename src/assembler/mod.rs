use crate::instruction::Opcode;
pub mod opcode_parser;
pub mod instruction_parsers;
pub mod program_parser;
pub mod register_parser;
pub mod operand_parser;
pub mod label_parser;
pub mod directive_parser;
use nom::types::CompleteStr;
use crate::assembler::program_parser::{program, Program};
use crate::vm;
// use crate::assembler::opcode::opcode_parsers;
// use crate::opcode::operand_parsers;
// use crate::opcode::register_parsers;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Op{code: Opcode},
    Register{reg_num: u8},
    IntegerOperand{value: i32},
    LabelDeclaration { name: String },
    LabelUsage { name: String },
    Directive { name: String }
}


#[derive(Debug)]
pub struct Assembler {
    pub phase: AssemblerPhase,
    pub symbols: SymbolTable
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            phase: AssemblerPhase::First,
            symbols: SymbolTable::new()
        }
    }
    // The assemble function accepts a raw string reference

    // Assembler gives the raw text to the program parser
    
    // It uses a match statement to check that the program parsed correctly
    
    // Assuming it did parse, we feed the program through each of the assembler phases
    
    // The assembler phases are broken out into other functions to help keep it neat
    
    // The first phase extracts all the labels and builds the symbol table
    
    // It then switches the phase to second
    
    // The second phase is then called, which just calls to_bytes on every AssemblerInstruction
    
    // All the bytes are added to a Vec<u8> which contains the fully assembled bytecode
    pub fn assemble(&mut self, raw: &str) -> Option<Vec<u8>> {
        match program(CompleteStr(raw)) {
            Ok((_remainder, program)) => {
                self.process_first_phase(&program);
                Some(self.process_second_phase(&program))
            },
            Err(e) => {
                println!("There was an error assembling the code: {:?}", e);
                None
            }
        }
    }
    
    fn process_first_phase(&mut self, p: &Program) {
        self.extract_labels(p);
        self.phase = AssemblerPhase::Second;
    }
    
    fn process_second_phase(&mut self, p: &Program) -> Vec<u8> {
        let mut program = vec![];
        for i in &p.instructions {
            let mut bytes = i.to_bytes();
            program.append(&mut bytes);

        }
        program
    }
    //extract labels
    fn extract_labels(&mut self, p: &Program) {
        let mut c = 0;
        for i in &p.instructions {
            if i.is_label() {
                match i.get_label_name() {
                    Some(name) => {
                        let symbol = Symbol::new(name, SymbolType::Label, c);
                        self.symbols.add_symbol(symbol);
                    },
                    None => {}
                };
            }
            c += 4;
        }
    }
}

#[derive(Debug)]
pub struct Symbol {
    name: String,
    offset: u32,
    symbol_type: SymbolType,
}

impl Symbol {
    pub fn new(name: String, symbol_type: SymbolType, offset: u32) -> Symbol {
        Symbol{
            name,
            symbol_type,
            offset
        }
    }
}

#[derive(Debug)]
pub enum SymbolType {
    Label,
}

#[derive(Debug)]
pub struct SymbolTable {
    symbols: Vec<Symbol>
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable{
            symbols: vec![]
        }
    }

    pub fn add_symbol(&mut self, s: Symbol) {
        self.symbols.push(s);
    }

    pub fn symbol_value(&self, s: &str) -> Option<u32> {
        for symbol in &self.symbols {
            if symbol.name == s {
                return Some(symbol.offset);
            }
        }
        None
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AssemblerPhase {
    First,
    Second,
}

#[test]
fn test_symbol_table() {
    let mut sym = SymbolTable::new();
    let new_symbol = Symbol::new("test".to_string(), SymbolType::Label, 12);
    sym.add_symbol(new_symbol);
    assert_eq!(sym.symbols.len(), 1);
    let v = sym.symbol_value("test");
    assert_eq!(true, v.is_some());
    let v = v.unwrap();
    assert_eq!(v, 12);
    let v = sym.symbol_value("does_not_exist");
    assert_eq!(v.is_some(), false);
}

#[test]
fn test_assemble_program() {
    let mut asm = Assembler::new();
    let test_string = "load $0 #100\nload $1 #1\nload $2 #0\ntest: inc $0\nneq $0 $2\njmpe @test\nhlt";
    let program = asm.assemble(test_string).unwrap();
    let mut vm = vm::VM::new();
    assert_eq!(program.len(), 24);
    for i in program{
        vm.add_byte(i);
    }
    assert_eq!(vm.program.len(), 24);
}