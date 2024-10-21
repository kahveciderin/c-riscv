use compile::{Compile, CompilerState};

pub mod compile;
mod instruction;
mod values;

pub fn compile_program(program: impl Compile) -> Vec<instruction::Instruction> {
    let mut state = CompilerState::new();
    program.compile(&mut state)
}
