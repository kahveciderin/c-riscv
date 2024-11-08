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
            UnaryOp::Nothing(expression) => {
                instructions.extend(expression.compile(state));
            }
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
            UnaryOp::Ref(expression) => {
                let lvalue = expression
                    .as_lvalue(state)
                    .unwrap_or_else(|| panic!("Cannot get a reference to non-lvalue"));
                instructions.extend(lvalue);
            }
            UnaryOp::Deref(expression) => {
                instructions.extend(expression.compile(state));

                instructions.push(Instruction::Lw(
                    Register::A0,
                    RegisterWithOffset(0.into(), Register::A0),
                ));
            }
        };

        instructions
    }
}

macro_rules! binary_op_inner {
    ($instructions:ident, $lhs:ident, $rhs:ident, $compiler_state:ident, $body:block) => {{
        $instructions.extend($lhs.compile($compiler_state));
        $instructions.push(Instruction::PushP(Register::A0));
        $instructions.extend($rhs.compile($compiler_state));
        $instructions.push(Instruction::PopP(Register::A1));

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
                instructions.push(Instruction::PushP(Register::A0));

                let lvalue = lhs
                    .as_lvalue(state)
                    .unwrap_or_else(|| panic!("Cannot assign to non-lvalue"));

                instructions.extend(lvalue);
                instructions.push(Instruction::PopP(Register::A1));
                instructions.push(Instruction::Sw(
                    Register::A1,
                    RegisterWithOffset(0.into(), Register::A0),
                ))
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
                        },

                        Comma: {
                            // the correct value is already in a0, do absolutely nothing
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
                    instructions.push(Instruction::Lw(
                        Register::A0,
                        RegisterWithOffset(variable.address.into(), Register::Fp),
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
                // riscv integer calling convention states that the first 8 arguments
                // should reside in a0-a7. If there are more than 8 arguments, the
                // remaining arguments are allowed to leak into the stack

                let stack_arguments = call.arguments.iter().skip(8);
                let register_arguments = call.arguments.iter().take(8).rev();

                let stack_argument_size =
                    nearest_multiple(4 * stack_arguments.len() as u32, 16) as i32;
                let register_argument_size =
                    nearest_multiple(4 * register_arguments.len() as u32, 16) as i32;

                instructions.push(Instruction::Addi(
                    Register::Sp,
                    Register::Sp,
                    (-stack_argument_size).into(),
                ));

                instructions.push(Instruction::Add(Register::S1, Register::Sp, Register::Zero));

                instructions.push(Instruction::Addi(
                    Register::Sp,
                    Register::Sp,
                    (-register_argument_size).into(),
                ));

                let mut current_address = 0;
                for arg in stack_arguments {
                    instructions.extend(arg.compile(state));

                    instructions.push(Instruction::Sw(
                        Register::A0,
                        RegisterWithOffset(current_address.into(), Register::S1),
                    ));

                    current_address += 4;
                }

                let mut current_address = 0;
                for arg in register_arguments.clone() {
                    instructions.extend(arg.compile(state));

                    instructions.push(Instruction::Sw(
                        Register::A0,
                        RegisterWithOffset(current_address.into(), Register::Sp),
                    ));

                    current_address += 4;
                }

                instructions.extend(call.expression.compile(state));
                instructions.push(Instruction::Add(Register::T0, Register::A0, Register::Zero));

                // currently, all arguments are pushed onto the stack
                // register arguments are in reverse order, and on the top
                // of the stack, while the stack arguments are in the correct
                // order, and at the bottom of the stack

                for (argument_count, _) in register_arguments.clone().enumerate() {
                    current_address -= 4;

                    instructions.push(Instruction::Lw(
                        match argument_count {
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
                        RegisterWithOffset(current_address.into(), Register::Sp),
                    ));
                }

                instructions.push(Instruction::Addi(
                    Register::Sp,
                    Register::Sp,
                    (register_argument_size).into(),
                ));

                // t0 contains the address of the function to call

                instructions.push(Instruction::Jalr(
                    Register::Ra,
                    RegisterWithOffset(0.into(), Register::T0),
                ));

                instructions.push(Instruction::Addi(
                    Register::Sp,
                    Register::Sp,
                    (stack_argument_size).into(),
                ));
            }
        };

        instructions
    }
}

trait AsLhs {
    fn as_lvalue(&self, state: &mut CompilerState) -> Option<Vec<Instruction>>;
}

impl AsLhs for Expression {
    fn as_lvalue(&self, state: &mut CompilerState) -> Option<Vec<Instruction>> {
        match self {
            Expression::Number(_)
            | Expression::BinaryOp(_)
            | Expression::FunctionSymbol(_)
            | Expression::TernaryOp(_)
            | Expression::Call(_) => None,

            Expression::Variable(name) => {
                if let Some(variable) = state.get_variable(name) {
                    Some(vec![Instruction::Addi(
                        Register::A0,
                        Register::Fp,
                        variable.address.into(),
                    )])
                } else {
                    None
                }
            }
            Expression::UnaryOp(op) => match op {
                UnaryOp::Deref(expression) => Some(expression.compile(state)),
                _ => None,
            },
        }
    }
}
