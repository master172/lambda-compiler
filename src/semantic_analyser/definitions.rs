use crate::{
    parser::definitions::{
        AbstractionASTNode, ApplicationASTNode,
        Atom::{AtomicExpression, Identifier},
        AtomicASTNode, DefinitionASTNode, ExpressionASTNode,
        Expressions::{Abstraction, Application},
        IdentifierASTNode, ProgramASTNode, StatementASTNode,
        Statemets::{Definition, ImportStatement, StatementExpression},
    },
    semantic_analyser::definitions::SemanticErrors::{DuplicateDefinition, UndeclaredIdentifier},
    visitor::definitons::Visitable,
};
use std::{collections::HashMap, fmt::Display};

use crate::visitor::definitons::Visitor;

#[derive(Debug)]
pub enum SemanticErrors {
    UndeclaredIdentifier(String),
    DuplicateDefinition(String),
}

impl Display for SemanticErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UndeclaredIdentifier(name) => write!(f, "undeclared identifer: {name}"),
            DuplicateDefinition(name) => write!(f, "duplicate definiton: {name}"),
        }
    }
}

pub struct SemanticTable {
    pub definitions: HashMap<String, DefinitionASTNode>,
}

pub struct Analyser<'table, 'ast> {
    semantic_table: &'table SemanticTable,
    bound_variables: Vec<&'ast String>,
    allow_free_variables: bool,
}

impl<'table, 'ast> Analyser<'table, 'ast> {
    pub fn new(table: &'table SemanticTable, allow_free_variables: bool) -> Self {
        Self {
            semantic_table: table,
            bound_variables: vec![],
            allow_free_variables,
        }
    }
}

impl<'table, 'ast> Visitor<'ast, (), SemanticErrors> for Analyser<'table, 'ast> {
    fn visit_program(&mut self, node: &'ast ProgramASTNode) -> Result<(), SemanticErrors> {
        //validate each statement errors are bubbled up
        for i in node.statements.iter() {
            i.accept(self)?;
        }
        Ok(())
    }

    fn visit_statement(&mut self, node: &'ast StatementASTNode) -> Result<(), SemanticErrors> {
        //just delegation
        match &node.statement {
            Definition(definition) => definition.accept(self)?,
            StatementExpression(expression) => expression.accept(self)?,
            ImportStatement(import) => import.accept(self)?,
        }
        Ok(())
    }

    fn visit_import(
        &mut self,
        node: &'ast crate::parser::definitions::ImportASTNode,
    ) -> Result<(), SemanticErrors> {
        Ok(())
    }

    fn visit_definition(&mut self, node: &'ast DefinitionASTNode) -> Result<(), SemanticErrors> {
        // definitions are already checked for duplicates in the first pass,
        // so just validate the expression
        let expression = &node.expression;
        expression.accept(self)?;
        Ok(())
    }

    fn visit_expression(&mut self, node: &'ast ExpressionASTNode) -> Result<(), SemanticErrors> {
        //just delegation again
        match &node.expression {
            Abstraction(abstraction) => abstraction.accept(self)?,
            Application(application) => application.accept(self)?,
        }
        Ok(())
    }

    fn visit_abstraction(&mut self, node: &'ast AbstractionASTNode) -> Result<(), SemanticErrors> {
        //this is mostly delegation again and some variable bound checking
        self.bound_variables.push(&node.identifier.name);
        let result = match &node.expression.as_ref() {
            Abstraction(abstraction) => abstraction.accept(self),
            Application(application) => application.accept(self),
        };
        self.bound_variables.pop();
        result
    }

    fn visit_application(&mut self, node: &'ast ApplicationASTNode) -> Result<(), SemanticErrors> {
        //applications are simple delegations to atomic
        node.left.accept(self)?;
        for i in node.right.iter() {
            i.accept(self)?;
        }
        Ok(())
    }

    fn visit_atomic(&mut self, node: &'ast AtomicASTNode) -> Result<(), SemanticErrors> {
        //atomics are also just delegation, either it is an identifier or an expression
        match &node.atom {
            Identifier(identifer) => identifer.accept(self)?,
            AtomicExpression(expression) => expression.accept(self)?,
        }
        Ok(())
    }

    fn visit_identifier(&mut self, node: &'ast IdentifierASTNode) -> Result<(), SemanticErrors> {
        // check a compiler flag
        if self.allow_free_variables {
            return Ok(());
        }
        // identifiers are the most important since all validation happens here
        // this should be obvious since the rest is just delegation
        // but thankfully it is easy
        // kust check if the identifier name exsists in the bound variables or the definitons table
        if !self.semantic_table.definitions.contains_key(&node.name)
            && !self.bound_variables.contains(&&node.name)
        {
            return Err(UndeclaredIdentifier(node.name.to_owned()));
        }

        Ok(())
    }
}
