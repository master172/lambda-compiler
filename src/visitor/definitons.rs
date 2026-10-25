use crate::parser::definitions::{
    AbstractionASTNode, ApplicationASTNode, AtomicASTNode, DefinitionASTNode, ExpressionASTNode,
    IdentifierASTNode, ProgramASTNode, StatementASTNode,
};

pub trait Visitor<E> {
    fn visit_program(&mut self, node: &ProgramASTNode) -> Result<(), E>;
    fn visit_statement(&mut self, node: &StatementASTNode) -> Result<(), E>;
    fn visit_expression(&mut self, node: &ExpressionASTNode) -> Result<(), E>;
    fn visit_definition(&mut self, node: &DefinitionASTNode) -> Result<(), E>;
    fn visit_abstraction(&mut self, node: &AbstractionASTNode) -> Result<(), E>;
    fn visit_application(&mut self, node: &ApplicationASTNode) -> Result<(), E>;
    fn visit_identifier(&mut self, node: &IdentifierASTNode) -> Result<(), E>;
    fn visit_atomic(&mut self, node: &AtomicASTNode) -> Result<(), E>;
}

pub trait Visitable {
    fn accept<V, E>(&self, visitor: &mut V) -> Result<(), E>
    where
        V: Visitor<E>;
}
