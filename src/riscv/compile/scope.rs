use crate::{
    riscv::instruction::Instruction,
    types::scope::{Scope, ScopeItem},
};

use super::{Compile, CompilerState};

impl Compile for Scope {
    fn compile(&self, state: &mut CompilerState) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        instructions.extend(
            state.create_scope(
                self.scope_state
                    .get_variables()
                    .iter()
                    .map(|v| (v.unique_name.clone(), v.datatype.clone()))
                    .collect(),
            ),
        );

        println!("compiler state: {:#?}", state);

        for statement in &self.items {
            instructions.extend(statement.compile(state));
        }

        // todo: find a way to do the cleanup after a return / break / continue etc. as well (not destroying the scope though)
        instructions.extend(state.destroy_scope());

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
