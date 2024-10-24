use std::sync::Arc;

use crate::{
    riscv::{
        instruction::Instruction,
        values::{Immediate, Register, RegisterWithOffset},
    },
    types::expression::{BinaryOp, Expression, UnaryOp},
    utils::{nearest_multiple::nearest_multiple, random_name::unique_identifier},
};

use super::{Compile, CompilerState};

impl Compile for UnaryOp {
    fn compile(&self, state: &mut CompilerState) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        match self {
            UnaryOp::Negation(expression) => {
                instructions.extend(expression.compile(state));
                instructions.push(Instruction::Neg(Register::A0, Register::A0));
            }
            UnaryOp::Plus(expression) => {
                instructions.extend(expression.compile(state));
            }
            UnaryOp::LogicalNot(expression) => {
                instructions.extend(expression.compile(state));
                instructions.push(Instruction::SeqzP(Register::A0, Register::A0));
            }
            UnaryOp::BitwiseNot(expression) => {
                instructions.extend(expression.compile(state));
                instructions.push(Instruction::NotP(Register::A0, Register::A0));
            }
            UnaryOp::PrefixIncrement(expression) => {
                let equivalent = BinaryOp::AssignmentAddition(
                    expression.clone(),
                    Arc::new(Expression::Number(1)),
                );

                instructions.extend(equivalent.compile(state));
            }
            UnaryOp::PrefixDecrement(expression) => {
                let equivalent = BinaryOp::AssignmentSubtraction(
                    expression.clone(),
                    Arc::new(Expression::Number(1)),
                );

                instructions.extend(equivalent.compile(state));
            }
            UnaryOp::PostfixDecrement(expression) => {
                let equivalent = UnaryOp::PrefixDecrement(expression.clone());
                instructions.extend(equivalent.compile(state));
                instructions.push(Instruction::Addi(Register::A0, Register::A0, 1.into()));
            }
            UnaryOp::PostfixIncrement(expression) => {
                let equivalent = UnaryOp::PrefixIncrement(expression.clone());
                instructions.extend(equivalent.compile(state));
                instructions.push(Instruction::Addi(Register::A0, Register::A0, (-1).into()));
            }
        };

        instructions
    }
}

macro_rules! binary_op_inner {
    ($instructions:ident, $lhs:ident, $rhs:ident, $compiler_state:ident, $body:block) => {{
        $instructions.extend($lhs.compile($compiler_state));
        $instructions.extend($compiler_state.push_register_tmp(Register::A0));
        $instructions.extend($rhs.compile($compiler_state));
        $instructions.extend($compiler_state.pop_register_tmp(Register::A1));

        // at this point, A0 contains the value of rhs and A1 contains the value of lhs
        $body
    }};
}

macro_rules! match_binary_ops {
    ($instructions:ident, $op:expr, $compiler_state:ident, [$($name:ident : $body:block),*]) => {
        match $op {
            $(BinaryOp::$name(lhs, rhs) => {
                binary_op_inner!($instructions, lhs, rhs, $compiler_state, $body)
            })*

            _ => {}
        }
    };
}

impl Compile for BinaryOp {
    fn compile(&self, state: &mut CompilerState) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        match self {
            BinaryOp::LogicalAnd(lhs, rhs) => {
                let short_circuit_label = unique_identifier(Some("short_circuit_and"), None);

                // compute the lhs
                instructions.extend(lhs.compile(state));

                // if lhs is false, short circuit
                instructions.push(Instruction::BeqzP(
                    Register::A0,
                    Immediate::Label(short_circuit_label.clone()),
                ));

                // compute the rhs
                instructions.extend(rhs.compile(state));

                // if rhs is false, jump to the short circuit label
                instructions.push(Instruction::BeqzP(
                    Register::A0,
                    Immediate::Label(short_circuit_label.clone()),
                ));

                instructions.push(Instruction::LiP(Register::A0, 1.into()));
                instructions.push(Instruction::Label(short_circuit_label));
            }
            BinaryOp::LogicalOr(lhs, rhs) => {
                let short_circuit_label_1 = unique_identifier(Some("short_circuit_or_1"), None);
                let short_circuit_label_2 = unique_identifier(Some("short_circuit_or_2"), None);

                // compute the lhs
                instructions.extend(lhs.compile(state));

                // if lhs is true, short circuit
                instructions.push(Instruction::BnezP(
                    Register::A0,
                    Immediate::Label(short_circuit_label_1.clone()),
                ));

                // compute the rhs
                instructions.extend(rhs.compile(state));

                // if rhs is true, jump to the short circuit label
                instructions.push(Instruction::BnezP(
                    Register::A0,
                    Immediate::Label(short_circuit_label_1.clone()),
                ));

                instructions.push(Instruction::LiP(Register::A0, 0.into()));

                instructions.push(Instruction::JP(Immediate::Label(
                    short_circuit_label_2.clone(),
                )));

                instructions.push(Instruction::Label(short_circuit_label_1));
                instructions.push(Instruction::LiP(Register::A0, 1.into()));
                instructions.push(Instruction::Label(short_circuit_label_2));
            }
            BinaryOp::NotEquals(lhs, rhs) => {
                let equivalent = UnaryOp::LogicalNot(Arc::new(Expression::BinaryOp(
                    BinaryOp::Equals((*lhs).clone(), (*rhs).clone()),
                )))
                .compile(state);
                instructions.extend(equivalent);
            }
            BinaryOp::LessThan(lhs, rhs) => {
                let equivalent = UnaryOp::LogicalNot(Arc::new(Expression::BinaryOp(
                    BinaryOp::GreaterThanEquals(lhs.clone(), rhs.clone()),
                )));
                instructions.extend(equivalent.compile(state));
            }
            BinaryOp::LessThanEquals(lhs, rhs) => {
                let equivalent = BinaryOp::GreaterThan(rhs.clone(), lhs.clone());
                instructions.extend(equivalent.compile(state));
            }
            BinaryOp::GreaterThanEquals(lhs, rhs) => {
                let equivalent = BinaryOp::LogicalOr(
                    Arc::new(Expression::BinaryOp(BinaryOp::Equals(
                        lhs.clone(),
                        rhs.clone(),
                    ))),
                    Arc::new(Expression::BinaryOp(BinaryOp::GreaterThan(
                        lhs.clone(),
                        rhs.clone(),
                    ))),
                );
                instructions.extend(equivalent.compile(state));
            }

            BinaryOp::Assignment(lhs, rhs) => {
                instructions.extend(rhs.compile(state));
                if let Expression::Variable(name) = lhs.as_ref() {
                    if let Some(variable) = state.get_variable(name) {
                        let relative_location = state.get_stack_size() - variable.address;
                        println!("Variable: {:?}, location: {relative_location}", variable);

                        instructions.push(Instruction::Sw(
                            Register::A0,
                            RegisterWithOffset((relative_location as i32).into(), Register::Sp),
                        ));
                    } else {
                        // error
                        todo!("Assignment lhs variable not found");
                    }
                } else {
                    // error
                    todo!("Assignment lhs invalid error");
                }
            }

            BinaryOp::AssignmentAddition(lhs, rhs) => {
                let equivalent = BinaryOp::Assignment(
                    lhs.clone(),
                    Arc::new(Expression::BinaryOp(BinaryOp::Addition(
                        lhs.clone(),
                        rhs.clone(),
                    ))),
                );
                instructions.extend(equivalent.compile(state));
            }

            BinaryOp::AssignmentSubtraction(lhs, rhs) => {
                let equivalent = BinaryOp::Assignment(
                    lhs.clone(),
                    Arc::new(Expression::BinaryOp(BinaryOp::Subtraction(
                        lhs.clone(),
                        rhs.clone(),
                    ))),
                );
                instructions.extend(equivalent.compile(state));
            }

            BinaryOp::AssignmentMultiplication(lhs, rhs) => {
                let equivalent = BinaryOp::Assignment(
                    lhs.clone(),
                    Arc::new(Expression::BinaryOp(BinaryOp::Multiplication(
                        lhs.clone(),
                        rhs.clone(),
                    ))),
                );
                instructions.extend(equivalent.compile(state));
            }

            BinaryOp::AssignmentDivision(lhs, rhs) => {
                let equivalent = BinaryOp::Assignment(
                    lhs.clone(),
                    Arc::new(Expression::BinaryOp(BinaryOp::Division(
                        lhs.clone(),
                        rhs.clone(),
                    ))),
                );
                instructions.extend(equivalent.compile(state));
            }

            BinaryOp::AssignmentModulus(lhs, rhs) => {
                let equivalent = BinaryOp::Assignment(
                    lhs.clone(),
                    Arc::new(Expression::BinaryOp(BinaryOp::Modulus(
                        lhs.clone(),
                        rhs.clone(),
                    ))),
                );
                instructions.extend(equivalent.compile(state));
            }

            BinaryOp::AssignmentShiftLeft(lhs, rhs) => {
                let equivalent = BinaryOp::Assignment(
                    lhs.clone(),
                    Arc::new(Expression::BinaryOp(BinaryOp::LeftShift(
                        lhs.clone(),
                        rhs.clone(),
                    ))),
                );
                instructions.extend(equivalent.compile(state));
            }

            BinaryOp::AssignmentShiftRight(lhs, rhs) => {
                let equivalent = BinaryOp::Assignment(
                    lhs.clone(),
                    Arc::new(Expression::BinaryOp(BinaryOp::RightShift(
                        lhs.clone(),
                        rhs.clone(),
                    ))),
                );
                instructions.extend(equivalent.compile(state));
            }

            BinaryOp::AssignmentBitwiseAnd(lhs, rhs) => {
                let equivalent = BinaryOp::Assignment(
                    lhs.clone(),
                    Arc::new(Expression::BinaryOp(BinaryOp::BitwiseAnd(
                        lhs.clone(),
                        rhs.clone(),
                    ))),
                );
                instructions.extend(equivalent.compile(state));
            }

            BinaryOp::AssignmentBitwiseXor(lhs, rhs) => {
                let equivalent = BinaryOp::Assignment(
                    lhs.clone(),
                    Arc::new(Expression::BinaryOp(BinaryOp::BitwiseXor(
                        lhs.clone(),
                        rhs.clone(),
                    ))),
                );
                instructions.extend(equivalent.compile(state));
            }

            BinaryOp::AssignmentBitwiseOr(lhs, rhs) => {
                let equivalent = BinaryOp::Assignment(
                    lhs.clone(),
                    Arc::new(Expression::BinaryOp(BinaryOp::BitwiseOr(
                        lhs.clone(),
                        rhs.clone(),
                    ))),
                );
                instructions.extend(equivalent.compile(state));
            }

            _ => {
                match_binary_ops!(
                    instructions,
                    self,
                    state,
                    [
                        Addition:  {
                            instructions.push(Instruction::Add(Register::A0, Register::A1, Register::A0));
                        },
                        Subtraction: {
                            instructions.push(Instruction::Sub(Register::A0, Register::A1, Register::A0));
                        },
                        Multiplication: {
                            // todo: multiply high
                            instructions.push(Instruction::Mul(Register::A0, Register::A1, Register::A0));
                        },
                        Division: {
                            instructions.push(Instruction::Div(Register::A0, Register::A1, Register::A0));
                        },
                        Modulus: {
                            instructions.push(Instruction::Rem(Register::A0, Register::A1, Register::A0));
                        },
                        BitwiseAnd: {
                            instructions.push(Instruction::And(Register::A0, Register::A1, Register::A0));
                        },
                        BitwiseXor: {
                            instructions.push(Instruction::Xor(Register::A0, Register::A1, Register::A0));
                        },
                        BitwiseOr: {
                            instructions.push(Instruction::Or(Register::A0, Register::A1, Register::A0));
                        },

                        // todo: arithmetic shifts for signed numbers
                        LeftShift: {
                            instructions.push(Instruction::Sll(Register::A0, Register::A1, Register::A0));
                        },
                        RightShift: {
                            instructions.push(Instruction::Srl(Register::A0, Register::A1, Register::A0));
                        },

                        // todo: comparison for signed numbers
                        GreaterThan: {
                            instructions.push(Instruction::Sltu(Register::A0, Register::A0, Register::A1));
                        },

                        Equals: {
                            instructions.push(Instruction::SeqP(Register::A0, Register::A0, Register::A1));
                        }
                    ]
                );
            }
        }

        instructions
    }
}

impl Compile for Expression {
    fn compile(&self, state: &mut CompilerState) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        match self {
            Expression::Number(n) => {
                instructions.push(Instruction::LiP(Register::A0, (*n).into()));
            }
            Expression::UnaryOp(op) => {
                instructions.extend(op.compile(state));
            }
            Expression::BinaryOp(op) => {
                instructions.extend(op.compile(state));
            }
            Expression::FunctionSymbol(name) => {
                instructions.push(Instruction::LaP(
                    Register::A0,
                    Immediate::Label(name.clone()),
                ));
            }
            Expression::Variable(name) => {
                if let Some(variable) = state.get_variable(name) {
                    let relative_location = state.get_stack_size() - variable.address;
                    println!("Variable: {:?}, location: {relative_location}", variable);

                    instructions.push(Instruction::Lw(
                        Register::A0,
                        RegisterWithOffset((relative_location as i32).into(), Register::Sp),
                    ));
                } else {
                    // error
                    todo!("Variable not found");
                }
            }
            Expression::TernaryOp(op) => {
                instructions.extend(op.condition.compile(state));

                let end_of_ternary_label = unique_identifier(Some("ternary_end"), None);
                let start_of_else_label = unique_identifier(Some("ternary_else_start"), None);

                instructions.push(Instruction::BeqzP(
                    Register::A0,
                    Immediate::Label(start_of_else_label.clone()),
                ));

                instructions.extend(op.then_expr.compile(state));

                instructions.push(Instruction::JP(Immediate::Label(
                    end_of_ternary_label.clone(),
                )));

                instructions.push(Instruction::Label(start_of_else_label));

                instructions.extend(op.else_expr.compile(state));

                instructions.push(Instruction::Label(end_of_ternary_label));
            }
            Expression::Call(call) => {
                let argument_count = call.arguments.len() as u32;

                // todo: dynamic size
                let stack_increase = if argument_count > 8 {
                    nearest_multiple(argument_count * 4, 16) as usize
                } else {
                    0
                };

                instructions.extend(state.expand_stack(stack_increase));

                // riscv integer calling convention states that the first 8 arguments
                // should reside in a0-a7. If there are more than 8 arguments, the
                // remaining arguments are allowed to leak into the stack

                instructions.extend(call.expression.compile(state));
                instructions.push(Instruction::Add(Register::T0, Register::A0, Register::Zero));

                let mut current_address = 0;
                let mut current_argument_count = 0;
                for arg in &call.arguments {
                    instructions.extend(arg.compile(state));

                    if current_argument_count > 8 {
                        instructions.push(Instruction::Sd(
                            Register::A0,
                            RegisterWithOffset(current_address.into(), Register::Sp),
                        ));
                    } else {
                        instructions.push(match current_argument_count {
                            0 => Instruction::Add(Register::T1, Register::A0, Register::Zero), 
                            1 => Instruction::Add(Register::A1, Register::A0, Register::Zero),
                            2 => Instruction::Add(Register::A2, Register::A0, Register::Zero),
                            3 => Instruction::Add(Register::A3, Register::A0, Register::Zero),
                            4 => Instruction::Add(Register::A4, Register::A0, Register::Zero),
                            5 => Instruction::Add(Register::A5, Register::A0, Register::Zero),
                            6 => Instruction::Add(Register::A6, Register::A0, Register::Zero),
                            7 => Instruction::Add(Register::A7, Register::A0, Register::Zero),
                            _ => unreachable!(),
                        })
                    }

                    // todo: dynamic size
                    current_address += 4;
                    current_argument_count += 1;
                }

                instructions.push(Instruction::Add(Register::A0, Register::T1, Register::Zero));

                // the T0 register holds the address of the function
                // all the arguments are in the A0-A7 registers (or on the stack)

                instructions.push(Instruction::Jalr(
                    Register::Ra,
                    RegisterWithOffset(0.into(), Register::T0),
                ));
            }
        };

        instructions
    }
}
