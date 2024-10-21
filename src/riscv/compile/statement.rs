use crate::{
    riscv::{
        instruction::Instruction,
        values::{Register, RegisterWithOffset},
    },
    types::statement::{JumpStatement, Statement},
};

use super::{Compile, CompilerState};

impl Compile for Statement {
    fn compile(&self, state: &mut CompilerState) -> Vec<Instruction> {
        match self {
            Statement::Jump { statement } => statement.compile(state),
            Statement::Expression { expression } => expression.compile(state),
            Statement::Null => Vec::new(),
        }
    }
}

impl Compile for JumpStatement {
    fn compile(&self, state: &mut CompilerState) -> Vec<Instruction> {
        match self {
            JumpStatement::Return { expression } => {
                let mut instructions = Vec::new();

                if let Some(expression) = expression {
                    let expression_compiled = expression.compile(state);
                    instructions.extend(expression_compiled);
                }

                instructions.extend(state.return_from_function());

                instructions
            }
        }
    }
}
