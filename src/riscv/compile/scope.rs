use crate::{
    riscv::instruction::Instruction,
    types::scope::{Scope, ScopeItem},
};

use super::{Compile, CompilerState};

impl Compile for Scope {
    fn compile(&self, state: &mut CompilerState) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        for statement in &self.items {
            instructions.extend(statement.compile(state));
        }

        instructions
    }
}

impl Compile for ScopeItem {
    fn compile(&self, state: &mut CompilerState) -> Vec<Instruction> {
        match self {
            ScopeItem::Statement(statement) => statement.compile(state),
            ScopeItem::Declaration(declaration) => declaration.compile(state),
        }
    }
}
