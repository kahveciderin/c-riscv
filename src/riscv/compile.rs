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
pub enum CompilerVariableLocation {
    Leaked,
    Stack,
}

#[derive(Debug, Clone)]
pub struct CompilerVariable {
    pub name: String,
    pub address: i32,
    pub datatype: Datatype,
    pub location: CompilerVariableLocation,
}

#[derive(Debug, Clone)]
pub struct CompilerScope {
    pub variables: Vec<CompilerVariable>,
}

impl CompilerScope {
    fn get_total_variable_type_size(&self) -> usize {
        let size = self
            .variables
            .iter()
            .filter_map(|v| {
                if v.location == CompilerVariableLocation::Leaked {
                    None
                } else {
                    Some(v.datatype.size())
                }
            })
            .sum::<usize>() as u32;

        nearest_multiple(size, STACK_ALIGNMENT) as usize
    }

    pub fn variable_size(&self) -> usize {
        self.get_total_variable_type_size()
    }
}

#[derive(Debug)]
pub struct CompilerState {
    pub scope: CompilerScope,
}

impl CompilerState {
    pub fn new() -> Self {
        CompilerState {
            scope: CompilerScope {
                variables: Vec::new(),
            },
        }
    }

    pub fn return_from_function(&mut self) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        instructions.push(Instruction::Comment(String::from(
            "Shrinking stack for the locals",
        )));

        instructions.push(Instruction::Addi(
            Register::Sp,
            Register::Sp,
            (self.scope.variable_size() as i32).into(),
        ));

        instructions.push(Instruction::Comment(String::from(
            "Returning the saved variables",
        )));

        // restore the saved register 1
        instructions.push(Instruction::Lw(
            Register::S1,
            RegisterWithOffset(24.into(), Register::Sp),
        ));

        // restore the frame pointer
        instructions.push(Instruction::Lw(
            Register::Fp,
            RegisterWithOffset(16.into(), Register::Sp),
        ));

        // restore the return address
        instructions.push(Instruction::Lw(
            Register::Ra,
            RegisterWithOffset(0.into(), Register::Sp),
        ));

        // shrink the stack (for the saved variables)
        instructions.push(Instruction::Addi(Register::Sp, Register::Sp, 32.into()));

        // instructions.push(Instruction::Addi(
        //     Register::Sp,
        //     Register::Sp,
        //     (self.scope.argument_size() as i32).into(),
        // ));

        // return
        instructions.push(Instruction::RetP);

        instructions.push(Instruction::Comment(String::from("return finished")));

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
        instructions.push(Instruction::Addi(Register::Sp, Register::Sp, (-16).into()));
        instructions.push(Instruction::Sw(
            register,
            RegisterWithOffset(0.into(), Register::Sp),
        ));
        instructions
    }

    fn pop_register_tmp(&mut self, register: Register) -> Vec<Instruction> {
        let mut instructions = Vec::new();
        instructions.push(Instruction::Lw(
            register,
            RegisterWithOffset(0.into(), Register::Sp),
        ));
        instructions.push(Instruction::Addi(Register::Sp, Register::Sp, (16).into()));
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

        instructions.push(Instruction::Comment(
            "Compiler output generated with MY OWN COMPILER".to_owned(),
        ));
        instructions.push(Instruction::Comment(
            "If required, I can provide you with the source code".to_owned(),
        ));
        instructions.push(Instruction::Comment(
            "The compiler is entirely my own work, so this makes".to_owned(),
        ));
        instructions.push(Instruction::Comment(
            "the code below also my own work, which is not plagiarism.".to_owned(),
        ));

        for function in &self.functions {
            instructions.extend(function.compile(state));
        }

        println!("state {state:#?}");

        instructions
    }
}
