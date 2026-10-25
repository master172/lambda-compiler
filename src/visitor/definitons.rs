use crate::parser::definitions::{
    AbstractionASTNode, ApplicationASTNode, AtomicASTNode, DefinitionASTNode, ExpressionASTNode,
    IdentifierASTNode, ProgramASTNode, StatementASTNode,
};

pub trait Visitor<'ast, E> {
    fn visit_program(&mut self, node: &'ast ProgramASTNode) -> Result<(), E>;
    fn visit_statement(&mut self, node: &'ast StatementASTNode) -> Result<(), E>;
    fn visit_expression(&mut self, node: &'ast ExpressionASTNode) -> Result<(), E>;
    fn visit_definition(&mut self, node: &'ast DefinitionASTNode) -> Result<(), E>;
    fn visit_abstraction(&mut self, node: &'ast AbstractionASTNode) -> Result<(), E>;
    fn visit_application(&mut self, node: &'ast ApplicationASTNode) -> Result<(), E>;
    fn visit_identifier(&mut self, node: &'ast IdentifierASTNode) -> Result<(), E>;
    fn visit_atomic(&mut self, node: &'ast AtomicASTNode) -> Result<(), E>;
}

pub trait Visitable<'ast> {
    fn accept<V, E>(&'ast self, visitor: &mut V) -> Result<(), E>
    where
        V: Visitor<'ast, E>;
}
