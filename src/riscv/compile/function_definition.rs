use crate::{
    riscv::instruction::Instruction,
    types::{
        expression::Expression, function_definition::FunctionDefinition, statement::JumpStatement,
    },
};

use super::{Compile, CompilerState};

impl Compile for FunctionDefinition<'_> {
    fn compile(&self, state: &mut CompilerState) -> Vec<Instruction> {
        let mut instructions = vec![];

        instructions.push(Instruction::Symbol("globl ".to_string() + &self.name));
        instructions.push(Instruction::Label(self.name.into()));

        instructions.extend(
            state.create_function_scope(
                self.scope_state
                    .get_variables()
                    .iter()
                    .map(|v| (v.unique_name.clone(), v.datatype.clone()))
                    .collect(),
            ),
        );

        instructions.push(Instruction::Comment("Function body:".to_owned()));

        println!("Current compiler state: {state:#?}");

        let body = self.body.compile(state);
        instructions.extend(body);

        instructions.push(Instruction::Comment("Function epilogue".to_owned()));

        let implicit_return = JumpStatement::Return {
            expression: Some(Expression::Number(0)),
        }
        .compile(state);
        instructions.extend(implicit_return);

        state.decrease_stack_size(32);

        instructions
    }
}
