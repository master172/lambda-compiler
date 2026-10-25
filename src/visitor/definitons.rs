use crate::parser::definitions::{
    AbstractionASTNode, ApplicationASTNode, AtomicASTNode, DefinitionASTNode, ExpressionASTNode,
    IdentifierASTNode, ProgramASTNode, StatementASTNode,
};

pub trait Visitor<'ast, R, E> {
    fn visit_program(&mut self, node: &'ast ProgramASTNode) -> Result<R, E>;
    fn visit_statement(&mut self, node: &'ast StatementASTNode) -> Result<R, E>;
    fn visit_expression(&mut self, node: &'ast ExpressionASTNode) -> Result<R, E>;
    fn visit_definition(&mut self, node: &'ast DefinitionASTNode) -> Result<R, E>;
    fn visit_abstraction(&mut self, node: &'ast AbstractionASTNode) -> Result<R, E>;
    fn visit_application(&mut self, node: &'ast ApplicationASTNode) -> Result<R, E>;
    fn visit_identifier(&mut self, node: &'ast IdentifierASTNode) -> Result<R, E>;
    fn visit_atomic(&mut self, node: &'ast AtomicASTNode) -> Result<R, E>;
}

pub trait Visitable<'ast> {
    fn accept<V, R, E>(&'ast self, visitor: &mut V) -> Result<R, E>
    where
        V: Visitor<'ast, R, E>;
}
