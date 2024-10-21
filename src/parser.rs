use std::sync::Arc;

use winnow::{PResult, Parser, Stateful};

use crate::{
    riscv::compile::Compile, types::datatype::Datatype, utils::random_name::unique_identifier,
};

mod datatype;
mod declaration;
mod expression;
mod function_definition;
mod identifier;
mod number;
mod program;
mod scope;
mod statement;
mod trivial_tokens;
mod whitespace;

mod binary_operation;

#[derive(Debug, Clone)]
pub struct ParserVariable {
    name: String,
    pub unique_name: String,
    pub datatype: Datatype,
}

#[derive(Debug, Clone)]
pub struct ParserScopeState {
    variables: Vec<Arc<ParserVariable>>,
}

impl ParserScopeState {
    pub fn new() -> Self {
        ParserScopeState { variables: vec![] }
    }

    pub fn add_variable(&mut self, name: String, datatype: Datatype) -> Arc<ParserVariable> {
        let unique_name = unique_identifier(Some(name.as_str()), None);
        let variable = ParserVariable {
            name,
            unique_name,
            datatype,
        };
        let variable = Arc::new(variable);
        self.insert_variable(variable.clone());
        variable
    }

    pub fn insert_variable(&mut self, variable: Arc<ParserVariable>) {
        self.variables.push(variable);
    }

    pub fn get_variable(&self, variable: &str) -> Option<Arc<ParserVariable>> {
        self.variables.iter().find(|v| v.name == variable).cloned()
    }

    pub fn get_variables(&self) -> Vec<Arc<ParserVariable>> {
        // todo: figure out another way
        self.variables.clone()
    }
}

#[derive(Debug)]
pub struct LoopState {
    id: String,
}

#[derive(Debug)]
pub struct ParserState {
    scope: Vec<ParserScopeState>,
    function_scope: ParserScopeState,
    loop_state: Vec<LoopState>,
}

impl ParserState {
    pub fn new() -> Self {
        ParserState {
            scope: vec![ParserScopeState::new()],
            function_scope: ParserScopeState::new(),
            loop_state: vec![],
        }
    }

    pub fn push_loop(&mut self, t: String) -> String {
        let id = unique_identifier(Some(t.as_str()), None);
        self.loop_state.push(LoopState { id: id.clone() });
        id
    }

    pub fn pop_loop(&mut self) -> LoopState {
        self.loop_state.pop().unwrap()
    }

    pub fn get_loop(&self) -> Option<&LoopState> {
        self.loop_state.last()
    }

    pub fn push_scope(&mut self) {
        self.scope.push(ParserScopeState::new());
    }

    pub fn pop_scope(&mut self) -> ParserScopeState {
        self.scope.pop().unwrap()
    }

    pub fn get_current_scope(&mut self) -> &mut ParserScopeState {
        self.scope.last_mut().unwrap()
    }

    pub fn add_variable(&mut self, variable: String, datatype: Datatype) -> Arc<ParserVariable> {
        let variable = self.get_current_scope().add_variable(variable, datatype);
        self.function_scope.insert_variable(variable.clone());
        variable
    }

    pub fn start_function_scope(&mut self) {
        self.function_scope = ParserScopeState::new();
    }

    pub fn get_variable(&self, variable: &str) -> Option<Arc<ParserVariable>> {
        self.scope
            .iter()
            .rev()
            .find_map(|s| s.get_variable(variable))
    }
}

pub type Stream<'is> = Stateful<&'is str, ParserState>;

pub fn parse_program<'s>(input: &'s str) -> PResult<impl Compile + 's> {
    let mut stream = Stream {
        input,
        state: ParserState::new(),
    };

    let ast = program::parse_program.parse_next(&mut stream);

    println!("Generated AST: {:#?}", ast);

    ast
}
