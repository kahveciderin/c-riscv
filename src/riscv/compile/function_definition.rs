use crate::{
    riscv::{
        instruction::Instruction,
        values::{Register, RegisterWithOffset},
    },
    types::{
        expression::Expression, function_definition::FunctionDefinition, statement::JumpStatement,
    },
    utils::nearest_multiple::nearest_multiple,
};

use super::{Compile, CompilerState, FunctionVariableType};

impl Compile for FunctionDefinition<'_> {
    fn compile(&self, state: &mut CompilerState) -> Vec<Instruction> {
        let mut instructions = vec![];

        instructions.push(Instruction::Symbol("globl ".to_string() + &self.name));
        instructions.push(Instruction::Label(self.name.into()));

        let mut function_variables: Vec<_> = self
            .scope_state
            .get_only_variables()
            .iter()
            .map(|v| {
                (
                    v.unique_name.clone(),
                    v.datatype.clone(),
                    FunctionVariableType::Local,
                )
            })
            .collect();

        // currently, the function arguments are from a0-7 and the rest
        // are leaked on the stack. we need to somehow put everything back on
        // the stack
        let register_arguments_size = self
            .arguments
            .iter()
            .take(8)
            .map(|a| a.datatype.size() as u32)
            .sum::<u32>();
        let register_arguments_size_aligned = nearest_multiple(register_arguments_size, 16) as i32;

        instructions.push(Instruction::Comment(
            "Operations for arguments passed by registers".to_owned(),
        ));

        // extend our stack just for the register arguments
        instructions.push(Instruction::Addi(
            Register::Sp,
            Register::Sp,
            (-register_arguments_size_aligned).into(),
        ));

        // this isn't aware of the above, so this points to the 9th argument
        let mut current_address = state.get_stack_size();
        let mut current_argument = 0;
        for arg in self.arguments.iter() {
            if current_argument <= 8 {
                instructions.push(Instruction::Sw(
                    match current_argument {
                        0 => Register::A0,
                        1 => Register::A1,
                        2 => Register::A2,
                        3 => Register::A3,
                        4 => Register::A4,
                        5 => Register::A5,
                        6 => Register::A6,
                        7 => Register::A7,
                        _ => unreachable!(),
                    },
                    RegisterWithOffset((current_argument as i32).into(), Register::Sp),
                ));
                function_variables.push((
                    arg.unique_name.clone(),
                    arg.datatype.clone(),
                    FunctionVariableType::Argument(current_address),
                ));
                current_address += arg.datatype.size();
            } else {
                todo!("stack leaking arguments")
            }

            current_argument += 1;
        }

        instructions.extend(state.create_function_scope(function_variables));

        instructions.push(Instruction::Comment("Function body:".to_owned()));

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
