use crate::{
    riscv::{
        instruction::Instruction,
        values::{Register, RegisterWithOffset},
    },
    types::{
        expression::Expression,
        function_definition::{FunctionDeclaration, FunctionDefinition},
        statement::JumpStatement,
    },
    utils::nearest_multiple::nearest_multiple,
};

use super::{Compile, CompilerState, CompilerVariable, CompilerVariableLocation};

impl Compile for FunctionDefinition<'_> {
    fn compile(&self, state: &mut CompilerState) -> Vec<Instruction> {
        let mut instructions = vec![];

        instructions.push(Instruction::Symbol("globl ".to_string() + &self.name));
        instructions.push(Instruction::Label(self.name.into()));

        instructions.push(Instruction::Comment("Function Prologue".to_owned()));

        // expand the stack
        instructions.push(Instruction::Addi(Register::Sp, Register::Sp, (-32).into()));

        // store the return address
        instructions.push(Instruction::Sw(
            Register::Ra,
            RegisterWithOffset(0.into(), Register::Sp),
        ));

        // store the frame pointer
        instructions.push(Instruction::Sw(
            Register::Fp,
            RegisterWithOffset(16.into(), Register::Sp),
        ));

        // store the saved register 1
        instructions.push(Instruction::Sw(
            Register::S1,
            RegisterWithOffset(24.into(), Register::Sp),
        ));

        instructions.push(Instruction::Comment(String::from(
            "Finished function prologue, now allocating space for variables",
        )));

        // handling variables

        let function_variables: Vec<_> = self.scope_state.get_only_variables();

        let variable_local_size: usize = function_variables.iter().map(|v| v.datatype.size()).sum();
        let register_argument_size = self.arguments.iter().take(8).count() * 4;
        let total_scope_size: usize = variable_local_size + register_argument_size;
        let stack_increase = nearest_multiple(total_scope_size as u32, 16) as i32;

        instructions.push(Instruction::Addi(
            Register::Sp,
            Register::Sp,
            (-stack_increase).into(),
        ));

        instructions.push(Instruction::Addi(Register::Fp, Register::Sp, 0.into()));

        // after this point, the frame pointer is the base of the stack

        state.scope.variables = Vec::new();

        let mut current_address = 0;
        let mut current_argument = 0;
        for register_variable in self.arguments.iter().take(8) {
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
                RegisterWithOffset((current_address).into(), Register::Fp),
            ));

            state.scope.variables.push(CompilerVariable {
                name: register_variable.unique_name.clone(),
                address: current_address,
                datatype: register_variable.datatype.clone(),
                location: CompilerVariableLocation::Stack,
            });

            current_argument += 1;
            current_address += 4; // todo: dynamic size
        }

        for variable in function_variables {
            let size = variable.datatype.size();
            let address = current_address;
            state.scope.variables.push(CompilerVariable {
                name: variable.unique_name.clone(),
                address,
                datatype: variable.datatype.clone(),
                location: CompilerVariableLocation::Stack,
            });

            instructions.push(Instruction::Comment("Variable ".to_owned() + &variable.unique_name + " at address " + &address.to_string()));

            current_address += size as i32;
        }

        let mut current_address = 32 + stack_increase;
        for stack_variable in self.arguments.iter().skip(8) {
            state.scope.variables.push(CompilerVariable {
                name: stack_variable.unique_name.clone(),
                address: current_address,
                datatype: stack_variable.datatype.clone(),
                location: CompilerVariableLocation::Leaked,
            });

            current_address += 4; // todo: dynamic size
        }

        instructions.push(Instruction::Comment(String::from(
            "Finished allocating space for variables",
        )));

        instructions.push(Instruction::Comment("Function body:".to_owned()));

        let body = self.body.compile(state);
        instructions.extend(body);

        instructions.push(Instruction::Comment("Function epilogue".to_owned()));

        let implicit_return = JumpStatement::Return {
            expression: Some(Expression::Number(0)),
        }
        .compile(state);
        instructions.extend(implicit_return);

        // state.decrease_stack_size(32);

        instructions
    }
}

impl Compile for FunctionDeclaration<'_> {
    fn compile(&self, state: &mut CompilerState) -> Vec<Instruction> {
        vec![]
    }
}
