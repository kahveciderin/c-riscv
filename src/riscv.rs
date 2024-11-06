use compile::{Compile, CompilerState};

pub mod compile;
mod instruction;
mod values;

pub fn compile_program(program: impl Compile) -> Vec<instruction::Instruction> {
    let mut state = CompilerState::new();
    program.compile(&mut state)
}

pub fn optimize_program(program: Vec<instruction::Instruction>) -> Vec<instruction::Instruction> {
    let mut optimized_program = Vec::new();

    for instruction in program.iter() {
        optimized_program.extend(instruction.convert_to_equivalent());
    }

    optimized_program
}
