use crate::{
    riscv::{
        instruction::Instruction,
        values::{Immediate, Register, RegisterWithOffset},
    },
    types::statement::{IfStatement, JumpStatement, Statement},
    utils::random_name::unique_identifier,
};

use super::{Compile, CompilerState};

impl Compile for Statement {
    fn compile(&self, state: &mut CompilerState) -> Vec<Instruction> {
        match self {
            Statement::Jump { statement } => statement.compile(state),
            Statement::Expression { expression } => expression.compile(state),
            Statement::Null => Vec::new(),
            Statement::Scope { scope } => scope.compile(state),
            Statement::If { statement } => statement.compile(state),
        }
    }
}

impl Compile for IfStatement {
    fn compile(&self, state: &mut CompilerState) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        instructions.extend(self.condition.compile(state));

        let end_of_if_label = unique_identifier(Some("if"), None);
        let start_of_else_label = unique_identifier(Some("if"), None);

        instructions.push(Instruction::Beqz(
            Register::A0,
            Immediate::Label(start_of_else_label.clone()),
        ));

        instructions.extend(self.then_block.compile(state));

        instructions.push(Instruction::J(Immediate::Label(end_of_if_label.clone())));

        instructions.push(Instruction::Label(start_of_else_label));

        if let Some(ref else_block) = self.else_block {
            instructions.extend(else_block.compile(state));
        }

        instructions.push(Instruction::Label(end_of_if_label));

        instructions
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
