use std::sync::Arc;

use crate::{
    parser::Case,
    riscv::{
        instruction::Instruction,
        values::{Immediate, Register},
    },
    types::{
        expression::Expression,
        scope::{Scope, ScopeItem},
        statement::{
            ForInit, ForStatement, IfStatement, JumpStatement, Statement, SwitchStatement,
            WhileStatement,
        },
    },
    utils::random_name::unique_identifier,
};

use super::{Compile, CompilerState};

impl Compile for Statement {
    fn compile(&self, state: &mut CompilerState) -> Vec<Instruction> {
        match self {
            Statement::Jump { statement } => statement.compile(state),
            Statement::Expression { expression } => expression.compile(state),
            Statement::Null => Vec::new(),
            Statement::Scope { scope } => scope.compile(state),
            Statement::If { statement } => statement.compile(state),
            Statement::While { statement } => statement.compile(state),
            Statement::For { statement } => statement.compile(state),
            Statement::Switch { statement } => statement.compile(state),
        }
    }
}

impl Compile for WhileStatement {
    fn compile(&self, state: &mut CompilerState) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        let while_start_label = self.id.clone() + "_start";
        let while_end_label = self.id.clone() + "_end";

        instructions.push(Instruction::Label(while_start_label.clone()));

        instructions.extend(self.condition.compile(state));

        instructions.push(Instruction::BeqzP(
            Register::A0,
            Immediate::Label(while_end_label.clone()),
        ));

        instructions.extend(self.block.compile(state));

        instructions.push(Instruction::JP(Immediate::Label(while_start_label.clone())));

        instructions.push(Instruction::Label(while_end_label));

        instructions
    }
}

impl Compile for ForStatement {
    fn compile(&self, state: &mut CompilerState) -> Vec<Instruction> {
        let init = if let Some(init) = self.init.clone() {
            match init {
                ForInit::Declaration(declaration) => ScopeItem::Declaration(declaration),
                ForInit::Expression(expression) => {
                    ScopeItem::Statement(Statement::Expression { expression })
                }
            }
        } else {
            ScopeItem::Statement(Statement::Null)
        };

        let condition = if let Some(condition) = self.condition.clone() {
            condition
        } else {
            Expression::Number(1)
        };

        let update = if let Some(update) = self.increment.clone() {
            Statement::Expression { expression: update }
        } else {
            Statement::Null
        };

        let inner_scope = (*self.block).clone();

        Scope {
            items: vec![
                init,
                ScopeItem::Statement(Statement::While {
                    statement: WhileStatement {
                        condition,
                        block: Arc::new(Statement::Scope {
                            scope: Scope {
                                items: vec![
                                    ScopeItem::Statement(update),
                                    ScopeItem::Statement(inner_scope),
                                ],
                            },
                        }),
                        id: self.id.clone(),
                    },
                }),
            ],
        }
        .compile(state)
    }
}

impl Compile for IfStatement {
    fn compile(&self, state: &mut CompilerState) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        instructions.extend(self.condition.compile(state));

        let end_of_if_label = unique_identifier(Some("if_end"), None);
        let start_of_else_label = unique_identifier(Some("else_start"), None);

        instructions.push(Instruction::BeqzP(
            Register::A0,
            Immediate::Label(start_of_else_label.clone()),
        ));

        instructions.extend(self.then_block.compile(state));

        instructions.push(Instruction::JP(Immediate::Label(end_of_if_label.clone())));

        instructions.push(Instruction::Label(start_of_else_label));

        if let Some(ref else_block) = self.else_block {
            instructions.extend(else_block.compile(state));
        }

        instructions.push(Instruction::Label(end_of_if_label));

        instructions
    }
}

impl Compile for JumpStatement {
    fn compile(&self, state: &mut CompilerState) -> Vec<Instruction> {
        match self {
            JumpStatement::Return { expression } => {
                let mut instructions = Vec::new();

                if let Some(expression) = expression {
                    let expression_compiled = expression.compile(state);
                    instructions.extend(expression_compiled);
                }

                instructions.extend(state.return_from_function());

                instructions
            }
            JumpStatement::Break { id } => {
                vec![Instruction::JP(Immediate::Label(id.clone() + "_end"))]
            }
            JumpStatement::Continue { id } => {
                vec![Instruction::JP(Immediate::Label(id.clone() + "_start"))]
            }
        }
    }
}

impl Compile for SwitchStatement {
    fn compile(&self, state: &mut CompilerState) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        instructions.extend(self.expression.compile(state));
        instructions.push(Instruction::Add(Register::S1, Register::A0, Register::Zero));

        for case in self.cases.iter() {
            if let Case::Case(case) = case {
                let number = Expression::Number(*case);
                instructions.extend(number.compile(state));
                instructions.push(Instruction::Beq(
                    Register::S1,
                    Register::A0,
                    Immediate::Label(self.id.clone() + "____case_" + &case.to_string()),
                ));
            }
        }
        for case in self.cases.iter() {
            if let Case::Default = case {
                instructions.push(Instruction::JP(Immediate::Label(
                    self.id.clone() + "____default",
                )));
            }
        }

        instructions.extend(self.body.compile(state));

        instructions.push(Instruction::Label(self.id.clone() + "_end"));

        instructions
    }
}
