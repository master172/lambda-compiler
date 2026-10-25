use crate::{
    lexer::definitions::SourceLocation,
    visitor::{
        self,
        definitons::{Visitable, Visitor},
    },
};
use std::fmt;

#[derive(Debug)]
pub struct ProgramASTNode {
    pub statements: Vec<StatementASTNode>,
}

#[derive(Debug)]
pub struct StatementASTNode {
    pub statement: Statemets,
}

#[derive(Debug)]
pub enum Statemets {
    Definition(DefinitionASTNode),
    StatementExpression(ExpressionASTNode),
}

#[derive(Debug)]
pub struct DefinitionASTNode {
    pub identifier: IdentifierASTNode,
    pub expression: ExpressionASTNode,
}

#[derive(Debug)]
pub struct ExpressionASTNode {
    pub expression: Expressions,
}

#[derive(Debug)]
pub struct IdentifierASTNode {
    pub name: String,
}

#[derive(Debug)]
pub enum Expressions {
    Abstraction(AbstractionASTNode),
    Application(ApplicationASTNode),
}

#[derive(Debug)]
pub struct AbstractionASTNode {
    pub identifier: IdentifierASTNode,
    pub expression: Box<Expressions>,
}

#[derive(Debug)]
pub struct ApplicationASTNode {
    pub left: Box<AtomicASTNode>,
    pub right: Vec<Box<AtomicASTNode>>,
}

#[derive(Debug)]
pub struct AtomicASTNode {
    pub atom: Atom,
}

#[derive(Debug)]
pub enum Atom {
    Identifier(IdentifierASTNode),
    AtomicExpression(ExpressionASTNode),
}

pub enum ParserErrors {
    UnexpectedEOFError,
    InvalidStatmentInitalizer { location: SourceLocation },
    InvalidDefinition { location: SourceLocation },
    InvalidExpression { location: SourceLocation },
    InvalidAbstraction { location: SourceLocation },
    InvalidAtomic { location: SourceLocation },
    ExpectedDotToken { location: SourceLocation },
    ExpectedSemiColon { location: SourceLocation },
    ExpectedRightParenthesisToken { location: SourceLocation },
    ExpectedIdentiferToken { location: SourceLocation },
    ExpectedAssignmentToken { location: SourceLocation },
}

impl fmt::Display for ParserErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserErrors::UnexpectedEOFError => {
                write!(f, "Unexpected end of file")
            }
            ParserErrors::InvalidStatmentInitalizer { location } => {
                write!(
                    f,
                    "Invalid statement initializer at line {}, column {}",
                    location.get_line() + 1,
                    location.get_columm() + 1
                )
            }
            ParserErrors::InvalidDefinition { location } => {
                write!(
                    f,
                    "Invalid definition at line {}, column {}",
                    location.get_line() + 1,
                    location.get_columm() + 1
                )
            }
            ParserErrors::InvalidExpression { location } => {
                write!(
                    f,
                    "Invalid expression at line {}, column {}",
                    location.get_line() + 1,
                    location.get_columm() + 1
                )
            }
            ParserErrors::InvalidAbstraction { location } => {
                write!(
                    f,
                    "Invalid abstraction at line {}, column {}",
                    location.get_line() + 1,
                    location.get_columm() + 1
                )
            }
            ParserErrors::InvalidAtomic { location } => {
                write!(
                    f,
                    "Invalid atomic expression at line {}, column {}",
                    location.get_line() + 1,
                    location.get_columm() + 1
                )
            }
            ParserErrors::ExpectedDotToken { location } => {
                write!(
                    f,
                    "Expected '.' at line {}, column {}",
                    location.get_line() + 1,
                    location.get_columm() + 1
                )
            }
            ParserErrors::ExpectedSemiColon { location } => {
                write!(
                    f,
                    "Expected ';' at line {}, column {}",
                    location.get_line() + 1,
                    location.get_columm() + 1
                )
            }
            ParserErrors::ExpectedRightParenthesisToken { location } => {
                write!(
                    f,
                    "Expected ')' at line {}, column {}",
                    location.get_line() + 1,
                    location.get_columm() + 1
                )
            }
            ParserErrors::ExpectedIdentiferToken { location } => {
                write!(
                    f,
                    "Expected an identifier at line {}, column {}",
                    location.get_line() + 1,
                    location.get_columm() + 1
                )
            }
            ParserErrors::ExpectedAssignmentToken { location } => {
                write!(
                    f,
                    "Expected '=' at line {}, column {}",
                    location.get_line() + 1,
                    location.get_columm() + 1
                )
            }
        }
    }
}

impl Visitable for ProgramASTNode {
    fn accept<V, E>(&self, visitor: &mut V) -> Result<(), E>
    where
        V: crate::visitor::definitons::Visitor<E>,
    {
        visitor.visit_program(self)
    }
}

impl Visitable for StatementASTNode {
    fn accept<V, E>(&self, visitor: &mut V) -> Result<(), E>
    where
        V: crate::visitor::definitons::Visitor<E>,
    {
        visitor.visit_statement(self)
    }
}

impl Visitable for DefinitionASTNode {
    fn accept<V, E>(&self, visitor: &mut V) -> Result<(), E>
    where
        V: crate::visitor::definitons::Visitor<E>,
    {
        visitor.visit_definition(self)
    }
}

impl Visitable for IdentifierASTNode {
    fn accept<V, E>(&self, visitor: &mut V) -> Result<(), E>
    where
        V: crate::visitor::definitons::Visitor<E>,
    {
        visitor.visit_identifier(self)
    }
}

impl Visitable for ExpressionASTNode {
    fn accept<V, E>(&self, visitor: &mut V) -> Result<(), E>
    where
        V: crate::visitor::definitons::Visitor<E>,
    {
        visitor.visit_expression(self)
    }
}

impl Visitable for AbstractionASTNode {
    fn accept<V, E>(&self, visitor: &mut V) -> Result<(), E>
    where
        V: crate::visitor::definitons::Visitor<E>,
    {
        visitor.visit_abstraction(self)
    }
}

impl Visitable for ApplicationASTNode {
    fn accept<V, E>(&self, visitor: &mut V) -> Result<(), E>
    where
        V: visitor::definitons::Visitor<E>,
    {
        visitor.visit_application(self)
    }
}

impl Visitable for AtomicASTNode {
    fn accept<V, E>(&self, visitor: &mut V) -> Result<(), E>
    where
        V: visitor::definitons::Visitor<E>,
    {
        visitor.visit_atomic(self)
    }
}
