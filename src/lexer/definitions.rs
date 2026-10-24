use std::fmt::Display;

use crate::lexer::definitions::LexErrors::{FailedToProcessLine, InvalidIdentifier};

#[derive(Debug)]
pub struct SourceLocation {
    line: usize,
    column: usize,
    size: usize,
}

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

    pub fn get_line_and_column(&self) -> (usize, usize) {
        (self.line, self.column)
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
    EOF,
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
