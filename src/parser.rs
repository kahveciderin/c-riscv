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
    pub datatype: Datatype,
}

#[derive(Debug, Clone)]
pub enum ParserSymbol {
    Variable(ParserVariable),
    Argument(ParserVariable),
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

    pub fn add_variable(&mut self, name: String, datatype: Datatype) -> ParserSymbol {
        let unique_name = unique_identifier(Some(name.as_str()), None);
        self.add_argument(ParserSymbol::Variable(ParserVariable {
            name: name.clone(),
            unique_name: unique_name.clone(),
            datatype: datatype.clone(),
        }))
    }

    pub fn add_argument(&mut self, variable: ParserSymbol) -> ParserSymbol {
        self.insert_variable(variable.clone());
        variable
    }

    pub fn insert_variable(&mut self, symbol: ParserSymbol) {
        self.symbols.push(Arc::new(symbol));
    }

    pub fn get_symbol(&self, variable: &str) -> Option<Arc<ParserSymbol>> {
        self.symbols
            .iter()
            .find(|v| {
                return match v.as_ref() {
                    ParserSymbol::Variable(v) => v.name == variable,
                    ParserSymbol::Argument(v) => v.name == variable,
                    ParserSymbol::Function(_) => false,
                };
            })
            .cloned()
    }

    pub fn get_only_variables(&self) -> Vec<ParserVariable> {
        // todo: figure out another way
        self.symbols
            .iter()
            .filter_map(|s| match s.as_ref() {
                ParserSymbol::Variable(v) => Some(v.clone()),
                ParserSymbol::Argument(_) => None,
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

    pub fn add_variable(&mut self, variable: String, datatype: Datatype) -> ParserSymbol {
        let variable = self.get_current_scope().add_variable(variable, datatype);
        self.function_scope.insert_variable(variable.clone());
        variable
    }

    pub fn add_argument(&mut self, symbol: ParserSymbol) -> ParserSymbol {
        let variable = self.get_current_scope().add_argument(symbol);
        self.function_scope.insert_variable(variable.clone());
        variable
    }

    pub fn add_static_symbol(&mut self, symbol: ParserStaticSymbol) {
        self.static_symbols.push(symbol);
    }

    pub fn start_function_scope(&mut self, name: String, arguments: Vec<Datatype>, ret: Datatype) {
        self.add_static_symbol(ParserStaticSymbol {
            name,
            datatype: Datatype::FunctionPointer {
                return_type: Arc::new(ret),
                arguments,
            },
        });
        self.function_scope = ParserScopeState::new();
    }

    pub fn get_symbol(&self, symbol: &str) -> Option<ParserSymbol> {
        self.scope
            .iter()
            .rev()
            .find_map(|s| s.get_symbol(symbol))
            .or_else(|| {
                self.static_symbols
                    .iter()
                    .find(|s| s.name == symbol)
                    .map(|s| Arc::new(ParserSymbol::Function(s.clone())).as_ref().clone())
                    .map(|f| Arc::new(f))
            })
            .map(|f| f.as_ref().clone())
    }

    pub fn get_by_unique_name(&self, unique_name: &str) -> Option<ParserSymbol> {
        self.scope
            .iter()
            .rev()
            .find_map(|s| {
                s.symbols
                    .iter()
                    .find(|v| match v.as_ref() {
                        ParserSymbol::Variable(v) => v.unique_name == unique_name,
                        ParserSymbol::Argument(v) => v.unique_name == unique_name,
                        ParserSymbol::Function(_) => false,
                    })
                    .cloned()
            })
            .or_else(|| {
                self.static_symbols
                    .iter()
                    .find(|s| s.name == unique_name)
                    .map(|s| Arc::new(ParserSymbol::Function(s.clone())))
            })
            .map(|f| f.as_ref().clone())
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
