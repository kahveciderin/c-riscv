use crate::{
    types::{datatype::Datatype, program::Program},
    utils::nearest_multiple::nearest_multiple,
};

use super::{
    instruction::Instruction,
    values::{Register, RegisterWithOffset},
};

const STACK_ALIGNMENT: u32 = 16;

#[derive(Debug, Clone)]
pub struct CompilerVariable {
    pub name: String,
    pub address: usize,
    pub datatype: Datatype,
}

#[derive(Debug, Clone)]
pub struct CompilerScope {
    pub variables: Vec<CompilerVariable>,
}

impl CompilerScope {
    pub fn size(&self) -> usize {
        self.variables.iter().map(|v| v.datatype.size()).sum()
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
        variables: Vec<(String, Datatype)>,
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

        let total_scope_size: usize = variables.iter().map(|v| v.1.size()).sum();
        let stack_increase = nearest_multiple(total_scope_size as u32, STACK_ALIGNMENT) as usize;

        instructions.extend(self.expand_stack(stack_increase));

        let mut current_stack_size = self.get_stack_size();

        for (name, datatype) in variables {
            let size = datatype.size();
            let address = current_stack_size;
            self.scope.variables.push(CompilerVariable {
                name,
                address,
                datatype: datatype.clone(),
            });
            current_stack_size -= size;
        }

        instructions.push(Instruction::Comment(String::from(
            "Finished allocating space for variables",
        )));

        instructions
    }

    pub fn return_from_function(&mut self) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        instructions.push(Instruction::Comment(
            "Destroying the stack we created for the variables".to_owned(),
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

        // shrink the stack
        instructions.extend(self.decrease_stack(32));

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

        let total_scope_size = scope.size();
        let stack_decrease = nearest_multiple(total_scope_size as u32, STACK_ALIGNMENT) as usize;

        instructions.extend(self.decrease_stack(stack_decrease));

        instructions
    }

    pub fn exit_scope(&mut self) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        let total_scope_size = self.scope.size();
        let increased_stack_size =
            nearest_multiple(total_scope_size as u32, STACK_ALIGNMENT) as usize;

        instructions.extend(self.decrease_stack(increased_stack_size));

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
