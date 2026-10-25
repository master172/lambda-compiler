use std::fmt::Display;

use crate::lexer::definitions::LexErrors::{FailedToProcessLine, InvalidIdentifier};

// the members of source location are kept private to prevent accidental mutation
#[derive(Debug, Clone, Copy)]
pub struct SourceLocation {
    line: usize,
    column: usize,
    size: usize,
}

// Source location members are only accessed through functions to prevent state corruption even accidentally
impl SourceLocation {
    pub fn new(line: usize, column: usize, size: usize) -> Self {
        Self { line, column, size }
    }

    pub fn get_line(&self) -> usize {
        self.line
    }

    pub fn get_columm(&self) -> usize {
        self.column
    }

    pub fn get_size(&self) -> usize {
        self.size
    }
}

#[derive(Debug)]
pub enum Token {
    Lambda {
        source: SourceLocation,
    },
    Dot {
        source: SourceLocation,
    },
    LeftParen {
        source: SourceLocation,
    },
    RightParen {
        source: SourceLocation,
    },
    Var {
        source: SourceLocation,
        attribute: String,
    },
    Assignment {
        source: SourceLocation,
    },
    SemiColon {
        source: SourceLocation,
    },
    EOF {
        source: SourceLocation,
    },
}

impl Token {
    pub fn get_source(&self) -> SourceLocation {
        match self {
            Self::EOF { source } => *source,
            Self::Assignment { source } => *source,
            Self::Dot { source } => *source,
            Self::Lambda { source } => *source,
            Self::LeftParen { source } => *source,
            Self::RightParen { source } => *source,
            Self::SemiColon { source } => *source,
            Self::Var {
                source,
                attribute: _attribute,
            } => *source,
        }
    }
}

pub enum LexErrors {
    InvalidIdentifier,
    FailedToProcessLine,
}

impl Display for LexErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidIdentifier => write!(f, "Invalid identifier name"),
            FailedToProcessLine => write!(f, "Failed to process line"),
        }
    }
}
