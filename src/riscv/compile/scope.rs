use crate::{
    riscv::instruction::Instruction,
    types::scope::{Label, Scope, ScopeItem},
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
            ScopeItem::Label(label) => label.compile(state),
        }
    }
}

impl Compile for Label {
    fn compile(&self, _state: &mut CompilerState) -> Vec<Instruction> {
        match self {
            Label::Named(_) => {
                todo!("named labels");
            }
            Label::Case { id, value } => {
                let mut instructions = Vec::new();
                let label = id.to_owned() + "____case_" + &value.to_string();
                instructions.push(Instruction::Label(label));

                instructions
            }
            Label::Default { id } => {
                let mut instructions = Vec::new();
                let label = id.to_owned() + "____default";
                instructions.push(Instruction::Label(label));

                instructions
            }
        }
    }
}
