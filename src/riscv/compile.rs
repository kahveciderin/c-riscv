use crate::{
    types::datatype::{self, Datatype},
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
    pub scope_type: CompilerScopeType,
}

impl CompilerScope {
    pub fn size(&self) -> usize {
        self.variables.iter().map(|v| v.datatype.size()).sum()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CompilerScopeType {
    Global,
    Function,
    Compound,
}

#[derive(Debug)]
pub struct CompilerState {
    pub scopes: Vec<CompilerScope>,
    pub stack_size: usize,
}

impl CompilerState {
    pub fn new() -> Self {
        CompilerState {
            scopes: vec![CompilerScope {
                variables: Vec::new(),
                scope_type: CompilerScopeType::Global,
            }],
            stack_size: 0,
        }
    }

    pub fn get_stack_size(&self) -> usize {
        self.stack_size
    }

    pub fn create_function_scope(&mut self, additional_stack_size: usize) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        self.push_scope(CompilerScopeType::Function);
        // expand the stack
        instructions.extend(self.expand_stack(additional_stack_size));

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
            "prologue finished, following is the body",
        )));

        instructions
    }

    pub fn return_from_function(&mut self) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        instructions.push(Instruction::Comment(String::from(
            "the following is a return instruction",
        )));

        let mut scopes_clone = self.scopes.clone();
        while scopes_clone.last().unwrap().scope_type != CompilerScopeType::Function {
            let current_scope = scopes_clone.pop().unwrap();
            instructions.extend(self.generate_delete_instructions_for_scope(current_scope));

            if self.scopes.last().unwrap().scope_type == CompilerScopeType::Global {
                break;
            }
        }

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
        instructions.push(Instruction::Ret);

        instructions.push(Instruction::Comment(String::from("return finished")));

        instructions
    }

    pub fn create_scope(&mut self, variables: Vec<(String, Datatype)>) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        self.push_scope(CompilerScopeType::Compound);
        let total_scope_size: usize = variables.iter().map(|v| v.1.size()).sum();
        let stack_increase = nearest_multiple(total_scope_size as u32, STACK_ALIGNMENT) as usize;

        instructions.extend(self.expand_stack(stack_increase));

        let mut current_stack_size = self.get_stack_size();

        for (name, datatype) in variables {
            let size = datatype.size();
            let address = current_stack_size;
            self.scopes
                .last_mut()
                .unwrap()
                .variables
                .push(CompilerVariable {
                    name,
                    address,
                    datatype: datatype.clone(),
                });
            current_stack_size -= size;
        }

        instructions
    }

    pub fn clean_scope_stack(&mut self) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        let scope = self.scopes.last().unwrap();
        let total_scope_size = scope.size();
        let stack_decrease = nearest_multiple(total_scope_size as u32, STACK_ALIGNMENT) as usize;

        instructions.extend(self.shrink_stack(stack_decrease));

        instructions
    }

    pub fn generate_delete_instructions_for_scope(
        &mut self,
        scope: CompilerScope,
    ) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        let total_scope_size = scope.size();
        let stack_decrease = nearest_multiple(total_scope_size as u32, STACK_ALIGNMENT) as usize;

        instructions.extend(self.decrease_stack(stack_decrease));

        instructions
    }

    pub fn destroy_scope(&mut self) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        let scope = self.pop_scope();
        let total_scope_size = scope.size();
        let increased_stack_size =
            nearest_multiple(total_scope_size as u32, STACK_ALIGNMENT) as usize;

        instructions.extend(self.shrink_stack(increased_stack_size));

        instructions
    }

    fn push_scope(&mut self, scope_type: CompilerScopeType) {
        self.scopes.push(CompilerScope {
            variables: Vec::new(),
            scope_type,
        });
    }

    fn pop_scope(&mut self) -> CompilerScope {
        self.scopes.pop().unwrap()
    }

    pub fn get_variable(&self, name: &str) -> Option<CompilerVariable> {
        for scope in self.scopes.iter().rev() {
            for variable in &scope.variables {
                if variable.name == name {
                    return Some(variable.clone());
                }
            }
        }

        None
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
