use std::fmt;

use crate::{
    arguement_processor::args_parser::{self, ArgsError},
    compiler_definitions::StateError::PathToIdentifierFailed,
    lexer::{self, definitions::LexErrors},
    parser::{self, definitions::ParserErrors},
};

pub enum StateError {
    PathToIdentifierFailed,
}

impl fmt::Display for StateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            PathToIdentifierFailed => write!(f, "failed to convert path to identifier string"),
        }
    }
}

pub enum CompilerErrors {
    ArgumentErrors(args_parser::ArgsError),
    ParserErrors(parser::definitions::ParserErrors),
    LexerErrors(lexer::definitions::LexErrors),
    StateErrors(StateError),
}

impl fmt::Display for CompilerErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArgumentErrors(err) => write!(f, "{err}"),
            Self::ParserErrors(err) => write!(f, "{err}"),
            Self::LexerErrors(err) => write!(f, "{err}"),
            Self::StateErrors(err) => write!(f, "{err}"),
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

impl From<StateError> for CompilerErrors {
    fn from(value: StateError) -> Self {
        Self::StateErrors(value)
    }
}
