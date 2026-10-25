use crate::parser::definitions::{
    AbstractionASTNode, ApplicationASTNode, AtomicASTNode, DefinitionASTNode, ExpressionASTNode,
    IdentifierASTNode, ProgramASTNode, StatementASTNode,
};

pub trait Visitor {
    fn visit_program(&mut self, node: &ProgramASTNode);
    fn visit_statement(&mut self, node: &StatementASTNode);
    fn visit_expression(&mut self, node: &ExpressionASTNode);
    fn visit_definition(&mut self, node: &DefinitionASTNode);
    fn visit_abstraction(&mut self, node: &AbstractionASTNode);
    fn visit_application(&mut self, node: &ApplicationASTNode);
    fn visit_identifier(&mut self, node: &IdentifierASTNode);
    fn visit_atomic(&mut self, node: &AtomicASTNode);
}

pub trait Visitable {
    fn accept<V>(&self, visitor: &mut V)
    where
        V: Visitor;
}
