use crate::{
    types::{datatype::Datatype, program::Program},
    utils::nearest_multiple::nearest_multiple,
};

use super::{
    instruction::Instruction,
    values::{Register, RegisterWithOffset},
};

const STACK_ALIGNMENT: u32 = 16;

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionVariableType {
    Local,
    Argument(usize), // address
}

#[derive(Debug, Clone)]
pub struct CompilerVariable {
    pub name: String,
    pub address: usize,
    pub datatype: Datatype,
    pub variable_type: FunctionVariableType,
}

#[derive(Debug, Clone)]
pub struct CompilerScope {
    pub variables: Vec<CompilerVariable>,
}

impl CompilerScope {
    fn get_total_variable_type_size(
        &self,
        variable_type: u32, // 0 is for local, 1 is for argument
    ) -> usize {
        let size = self
            .variables
            .iter()
            .filter_map(|v| {
                if variable_type
                    == match v.variable_type {
                        FunctionVariableType::Local => 0,
                        FunctionVariableType::Argument(_) => 1,
                    }
                {
                    Some(v.datatype.size())
                } else {
                    None
                }
            })
            .sum::<usize>() as u32;

        nearest_multiple(size, STACK_ALIGNMENT) as usize
    }

    pub fn variable_size(&self) -> usize {
        self.get_total_variable_type_size(0)
    }
    pub fn argument_size(&self) -> usize {
        self.get_total_variable_type_size(1)
    }

    pub fn size(&self) -> usize {
        let local_variables_size = self.get_total_variable_type_size(0);
        let argument_size = self.get_total_variable_type_size(1);

        local_variables_size + argument_size
    }
}

#[derive(Debug)]
pub struct CompilerState {
    pub scope: CompilerScope,
    pub stack_size: usize,
}

impl CompilerState {
    pub fn new() -> Self {
        CompilerState {
            scope: CompilerScope {
                variables: Vec::new(),
            },
            stack_size: 0,
        }
    }

    pub fn get_stack_size(&self) -> usize {
        self.stack_size
    }

    pub fn create_function_scope(
        &mut self,
        variables: Vec<(String, Datatype, FunctionVariableType)>,
    ) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        self.scope.variables = Vec::new();

        instructions.push(Instruction::Comment("Function Prologue".to_owned()));

        // expand the stack
        instructions.extend(self.expand_stack(32));

        // store the return address
        instructions.push(Instruction::Sd(
            Register::Ra,
            RegisterWithOffset(0.into(), Register::Sp),
        ));

        // store the frame pointer
        instructions.push(Instruction::Sd(
            Register::S0,
            RegisterWithOffset(16.into(), Register::Sp),
        ));

        // store the saved register 1
        instructions.push(Instruction::Sd(
            Register::S1,
            RegisterWithOffset(24.into(), Register::Sp),
        ));

        instructions.push(Instruction::Comment(String::from(
            "Finished function prologue, now allocating space for variables",
        )));

        let variable_local_size: usize = variables
            .iter()
            .filter_map(|v| {
                if v.2 == FunctionVariableType::Local {
                    Some(v.1.size())
                } else {
                    None
                }
            })
            .sum();
        let total_scope_size: usize = variable_local_size;
        let stack_increase = nearest_multiple(total_scope_size as u32, STACK_ALIGNMENT) as usize;

        println!("Stack increase: {stack_increase}, {variables:#?}");

        instructions.extend(self.expand_stack(stack_increase));

        let mut current_stack_size = self.get_stack_size();

        for (name, datatype, variable_type) in variables {
            let size = datatype.size();
            let address = if let FunctionVariableType::Argument(address) = variable_type {
                address
            } else {
                current_stack_size
            };
            self.scope.variables.push(CompilerVariable {
                name,
                address,
                datatype: datatype.clone(),
                variable_type: variable_type.clone(),
            });
            if let FunctionVariableType::Local = variable_type {
                current_stack_size -= size;
            }
        }

        instructions.push(Instruction::Comment(String::from(
            "Finished allocating space for variables",
        )));

        instructions
    }

    pub fn return_from_function(&mut self) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        instructions.push(Instruction::Comment(
            "Deleting the space we created for the variables".to_owned(),
        ));
        instructions.extend(self.exit_scope());

        instructions.push(Instruction::Comment(String::from(
            "Returning the saved variables",
        )));

        self.generate_delete_instructions_for_scope(&(self.scope.clone()));

        // restore the saved register 1
        instructions.push(Instruction::Ld(
            Register::S1,
            RegisterWithOffset(24.into(), Register::Sp),
        ));

        // restore the frame pointer
        instructions.push(Instruction::Ld(
            Register::S0,
            RegisterWithOffset(16.into(), Register::Sp),
        ));

        // restore the return address
        instructions.push(Instruction::Ld(
            Register::Ra,
            RegisterWithOffset(0.into(), Register::Sp),
        ));

        // shrink the stack (for the saved variables)
        instructions.extend(self.decrease_stack(32));

        instructions.extend(self.decrease_stack(self.scope.argument_size()));

        // return
        instructions.push(Instruction::RetP);

        instructions.push(Instruction::Comment(String::from("return finished")));

        instructions
    }

    pub fn generate_delete_instructions_for_scope(
        &mut self,
        scope: &CompilerScope,
    ) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        let total_scope_size = scope.variable_size();

        instructions.extend(self.decrease_stack(total_scope_size));

        instructions
    }

    pub fn exit_scope(&mut self) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        let total_scope_size = self.scope.variable_size();

        instructions.extend(self.decrease_stack(total_scope_size));

        instructions
    }

    pub fn get_variable(&self, name: &str) -> Option<CompilerVariable> {
        for variable in &self.scope.variables {
            if variable.name == name {
                return Some(variable.clone());
            }
        }

        None
    }

    fn push_register_tmp(&mut self, register: Register) -> Vec<Instruction> {
        let mut instructions = Vec::new();
        instructions.extend(self.expand_stack(16));
        instructions.push(Instruction::Sd(
            register,
            RegisterWithOffset(0.into(), Register::Sp),
        ));
        instructions
    }

    fn pop_register_tmp(&mut self, register: Register) -> Vec<Instruction> {
        let mut instructions = Vec::new();
        instructions.push(Instruction::Ld(
            register,
            RegisterWithOffset(0.into(), Register::Sp),
        ));
        instructions.extend(self.shrink_stack(16));
        instructions
    }

    fn expand_stack(&mut self, size: usize) -> Vec<Instruction> {
        println!("Expanding stack by {}", size);
        let mut instructions = Vec::new();
        let decrease_size = (-(size as i32)).into();
        instructions.push(Instruction::Addi(Register::Sp, Register::Sp, decrease_size));

        self.stack_size += size;

        instructions
    }

    pub fn decrease_stack_size(&mut self, size: usize) {
        self.stack_size -= size;
    }

    fn shrink_stack(&mut self, size: usize) -> Vec<Instruction> {
        println!("Shrinking stack by {}", size);
        self.decrease_stack_size(size);

        self.decrease_stack(size)
    }

    // this function doesn't touch the stack_size
    pub fn decrease_stack(&mut self, size: usize) -> Vec<Instruction> {
        println!("Decreasing stack by {}", size);
        let mut instructions = Vec::new();
        let increase_size = (size as i32).into();
        instructions.push(Instruction::Addi(Register::Sp, Register::Sp, increase_size));

        instructions
    }
}

pub trait Compile {
    fn compile(&self, state: &mut CompilerState) -> Vec<Instruction>;
}

mod declaration;
mod expression;
mod function_definition;
mod scope;
mod statement;

impl Compile for Program<'_> {
    fn compile(&self, state: &mut CompilerState) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        for function in &self.functions {
            instructions.extend(function.compile(state));
        }

        instructions
    }
}
