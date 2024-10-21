use crate::{
    riscv::{
        instruction::Instruction,
        values::{Register, RegisterWithOffset},
    },
    types::{
        expression::Expression, function_definition::FunctionDefinition, statement::JumpStatement,
    },
};

use super::{Compile, CompilerState};

impl Compile for FunctionDefinition<'_> {
    fn compile(&self, state: &mut CompilerState) -> Vec<Instruction> {
        let mut instructions = vec![];

        instructions.push(Instruction::Symbol("globl main".into()));
        instructions.push(Instruction::Label(self.name.into()));

        instructions.extend(state.create_function_scope(32));

        let body = self.body.compile(state);
        instructions.extend(body);

        instructions.push(Instruction::Comment(String::from(
            "body finished, following is the epilogue",
        )));

        let implicit_return = JumpStatement::Return {
            expression: Some(Expression::Number(0)),
        }
        .compile(state);
        instructions.extend(implicit_return);

        state.decrease_stack_size(32);

        instructions
    }
}
