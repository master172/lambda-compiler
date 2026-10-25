use crate::parser::definitions::{
    AbstractionASTNode, ApplicationASTNode, AtomicASTNode, DefinitionASTNode, ExpressionASTNode,
    IdentifierASTNode, ProgramASTNode, StatementASTNode,
};
use std::collections::HashMap;

use crate::visitor::definitons::Visitor;

pub enum SemanticErrors<'a> {
    UndeclaredIdentifier(&'a str),
    DuplicateDefinition(&'a str),
}

pub struct SemanticTable<'a> {
    pub definitions: HashMap<&'a str, &'a DefinitionASTNode>,
}

struct Analyser<'table, 'ast> {
    semantic_table: &'table SemanticTable<'ast>,
    bound_variables: Vec<String>,
}

impl<'table, 'ast> Analyser<'table, 'ast> {
    pub fn new(program_ast: &'ast ProgramASTNode, table: &'table SemanticTable<'ast>) -> Self {
        Self {
            semantic_table: table,
            bound_variables: vec![],
        }
    }
}

impl<'table, 'ast> Visitor for Analyser<'table, 'ast> {
    fn visit_abstraction(&mut self, node: &AbstractionASTNode) {
        todo!()
    }
    fn visit_application(&mut self, node: &ApplicationASTNode) {
        todo!()
    }
    fn visit_atomic(&mut self, node: &AtomicASTNode) {
        todo!()
    }
    fn visit_definition(&mut self, node: &DefinitionASTNode) {
        todo!()
    }
    fn visit_expression(&mut self, node: &ExpressionASTNode) {
        todo!()
    }
    fn visit_identifier(&mut self, node: &IdentifierASTNode) {
        todo!()
    }
    fn visit_program(&mut self, node: &ProgramASTNode) {
        todo!()
    }
    fn visit_statement(&mut self, node: &StatementASTNode) {
        todo!()
    }
}
