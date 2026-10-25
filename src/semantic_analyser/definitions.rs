use crate::{
    parser::definitions::{
        AbstractionASTNode, ApplicationASTNode, AtomicASTNode, DefinitionASTNode,
        ExpressionASTNode, IdentifierASTNode, ProgramASTNode, StatementASTNode,
        Statemets::{Definition, StatementExpression},
    },
    visitor::definitons::Visitable,
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

impl<'table, 'ast> Visitor<SemanticErrors<'ast>> for Analyser<'table, 'ast> {
    fn visit_program(&mut self, node: &ProgramASTNode) -> Result<(), SemanticErrors<'ast>> {
        for i in node.statements.iter() {
            i.accept(self)?;
        }
        Ok(())
    }

    fn visit_statement(&mut self, node: &StatementASTNode) -> Result<(), SemanticErrors<'ast>> {
        match &node.statement {
            Definition(definition) => definition.accept(self)?,
            StatementExpression(expression) => expression.accept(self)?,
        }
        Ok(())
    }

    fn visit_abstraction(&mut self, node: &AbstractionASTNode) -> Result<(), SemanticErrors<'ast>> {
        todo!()
    }
    fn visit_application(&mut self, node: &ApplicationASTNode) -> Result<(), SemanticErrors<'ast>> {
        todo!()
    }
    fn visit_atomic(&mut self, node: &AtomicASTNode) -> Result<(), SemanticErrors<'ast>> {
        todo!()
    }
    fn visit_definition(&mut self, node: &DefinitionASTNode) -> Result<(), SemanticErrors<'ast>> {
        todo!()
    }
    fn visit_expression(&mut self, node: &ExpressionASTNode) -> Result<(), SemanticErrors<'ast>> {
        todo!()
    }
    fn visit_identifier(&mut self, node: &IdentifierASTNode) -> Result<(), SemanticErrors<'ast>> {
        todo!()
    }
}
