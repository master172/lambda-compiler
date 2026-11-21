use std::{collections::HashMap, fmt};

use crate::{
    arguement_processor::args_parser::{self, ArgsError},
    lexer::{self, definitions::LexErrors},
    parser::{
        self,
        definitions::{ParserErrors, ProgramASTNode},
    },
};

pub enum CompilerErrors {
    ArgumentErrors(args_parser::ArgsError),
    ParserErrors(parser::definitions::ParserErrors),
    LexerErrors(lexer::definitions::LexErrors),
}

impl fmt::Display for CompilerErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArgumentErrors(err) => write!(f, "{err}"),
            Self::ParserErrors(err) => write!(f, "{err}"),
            Self::LexerErrors(err) => write!(f, "{err}"),
        }
    }
}

impl From<ArgsError> for CompilerErrors {
    fn from(value: ArgsError) -> Self {
        Self::ArgumentErrors(value)
    }
}

impl From<ParserErrors> for CompilerErrors {
    fn from(value: ParserErrors) -> Self {
        Self::ParserErrors(value)
    }
}

impl From<LexErrors> for CompilerErrors {
    fn from(value: LexErrors) -> Self {
        Self::LexerErrors(value)
    }
}

#[derive(Debug)]
pub struct ModuleStore {
    pub modules: HashMap<String, ProgramASTNode>,
}

impl ModuleStore {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }
}
