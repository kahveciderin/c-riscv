use std::sync::Arc;

use crate::{
    riscv::instruction::Instruction,
    types::{
        declaration::Declaration,
        expression::{BinaryOp, Expression},
    },
};

use super::{Compile, CompilerState};

impl Compile for Declaration {
    fn compile(&self, state: &mut CompilerState) -> Vec<Instruction> {
        let mut instructions = vec![];

        if let Some(ref value) = self.value {
            let equivalent = Expression::BinaryOp(BinaryOp::Assignment(
                Arc::new(Expression::Variable(self.name.clone())),
                Arc::new(value.clone()),
            ));
            instructions.extend(equivalent.compile(state));
        }

        instructions
    }
}
