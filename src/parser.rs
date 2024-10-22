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
pub struct ParserStaticSymbol {
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum ParserSymbol {
    Variable(ParserVariable),
    Function(ParserStaticSymbol),
}

#[derive(Debug, Clone)]
pub struct ParserScopeState {
    symbols: Vec<Arc<ParserSymbol>>,
}

impl ParserScopeState {
    pub fn new() -> Self {
        ParserScopeState { symbols: vec![] }
    }

    pub fn add_variable(&mut self, name: String, datatype: Datatype) -> ParserVariable {
        let unique_name = unique_identifier(Some(name.as_str()), None);
        self.add_raw_name_variable(name, unique_name, datatype)
    }

    pub fn add_raw_name_variable(
        &mut self,
        name: String,
        raw_name: String,
        datatype: Datatype,
    ) -> ParserVariable {
        let variable = ParserVariable {
            name,
            unique_name: raw_name,
            datatype,
        };
        self.insert_variable(variable.clone());
        variable
    }

    pub fn insert_variable(&mut self, variable: ParserVariable) {
        self.symbols
            .push(Arc::new(ParserSymbol::Variable(variable)));
    }

    pub fn get_symbol(&self, variable: &str) -> Option<Arc<ParserSymbol>> {
        self.symbols
            .iter()
            .find(|v| {
                return match v.as_ref() {
                    ParserSymbol::Variable(v) => v.name == variable,
                    ParserSymbol::Function(_) => false,
                };
            })
            .cloned()
    }

    pub fn get_variables(&self) -> Vec<ParserVariable> {
        // todo: figure out another way
        self.symbols
            .iter()
            .filter_map(|s| match s.as_ref() {
                ParserSymbol::Variable(v) => Some(v.clone()),
                ParserSymbol::Function(_) => None,
            })
            .collect()
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
    static_symbols: Vec<ParserStaticSymbol>,
    loop_state: Vec<LoopState>,
}

impl ParserState {
    pub fn new() -> Self {
        ParserState {
            scope: vec![ParserScopeState::new()],
            function_scope: ParserScopeState::new(),
            loop_state: vec![],
            static_symbols: vec![],
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

    pub fn add_variable(&mut self, variable: String, datatype: Datatype) -> ParserVariable {
        let variable = self.get_current_scope().add_variable(variable, datatype);
        self.function_scope.insert_variable(variable.clone());
        variable
    }

    pub fn add_raw_name_variable(
        &mut self,
        variable: String,
        raw_name: String,
        datatype: Datatype,
    ) -> ParserVariable {
        let variable = self
            .get_current_scope()
            .add_raw_name_variable(variable, raw_name, datatype);
        self.function_scope.insert_variable(variable.clone());
        variable
    }

    pub fn add_static_symbol(&mut self, symbol: ParserStaticSymbol) {
        self.static_symbols.push(symbol);
    }

    pub fn start_function_scope(&mut self, name: String) {
        self.add_static_symbol(ParserStaticSymbol { name });
        self.function_scope = ParserScopeState::new();
    }

    pub fn get_symbol(&self, symbol: &str) -> Option<ParserSymbol> {
        if let Some(symbol) = self
            .scope
            .iter()
            .rev()
            .find_map(|s| s.get_symbol(symbol))
            .map(|s| s.as_ref().clone())
        {
            return Some(symbol);
        } else if let Some(symbol) = self.static_symbols.iter().find(|s| s.name == symbol) {
            return Some(ParserSymbol::Function(symbol.clone()));
        } else {
            None
        }
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
