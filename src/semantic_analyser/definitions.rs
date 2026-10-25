use crate::{
    parser::definitions::{
        AbstractionASTNode, ApplicationASTNode,
        Atom::{AtomicExpression, Identifier},
        AtomicASTNode, DefinitionASTNode, ExpressionASTNode,
        Expressions::{Abstraction, Application},
        IdentifierASTNode, ProgramASTNode, StatementASTNode,
        Statemets::{Definition, StatementExpression},
    },
    semantic_analyser::definitions::SemanticErrors::{DuplicateDefinition, UndeclaredIdentifier},
    visitor::definitons::Visitable,
};
use std::{collections::HashMap, fmt::Display};

use crate::visitor::definitons::Visitor;

#[derive(Debug)]
pub enum SemanticErrors<'a> {
    UndeclaredIdentifier(&'a str),
    DuplicateDefinition(&'a str),
}

impl<'a> Display for SemanticErrors<'a> {
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
    bound_variables: Vec<&'ast str>,
}

impl<'table, 'ast> Analyser<'table, 'ast> {
    pub fn new(table: &'table SemanticTable) -> Self {
        Self {
            semantic_table: table,
            bound_variables: vec![],
        }
    }
}

impl<'table, 'ast> Visitor<'ast, (), SemanticErrors<'ast>> for Analyser<'table, 'ast> {
    fn visit_program(&mut self, node: &'ast ProgramASTNode) -> Result<(), SemanticErrors<'ast>> {
        //validate each statement errors are bubbled up
        for i in node.statements.iter() {
            i.accept(self)?;
        }
        Ok(())
    }

    fn visit_statement(
        &mut self,
        node: &'ast StatementASTNode,
    ) -> Result<(), SemanticErrors<'ast>> {
        //just delegation
        match &node.statement {
            Definition(definition) => definition.accept(self)?,
            StatementExpression(expression) => expression.accept(self)?,
        }
        Ok(())
    }

    fn visit_definition(
        &mut self,
        node: &'ast DefinitionASTNode,
    ) -> Result<(), SemanticErrors<'ast>> {
        // definitions are already checked for duplicates in the first pass,
        // so just validate the expression
        let expression = &node.expression;
        expression.accept(self)?;
        Ok(())
    }

    fn visit_expression(
        &mut self,
        node: &'ast ExpressionASTNode,
    ) -> Result<(), SemanticErrors<'ast>> {
        //just delegation again
        match &node.expression {
            Abstraction(abstraction) => abstraction.accept(self)?,
            Application(application) => application.accept(self)?,
        }
        Ok(())
    }

    fn visit_abstraction(
        &mut self,
        node: &'ast AbstractionASTNode,
    ) -> Result<(), SemanticErrors<'ast>> {
        //this is mostly delegation again and some variable bound checking
        self.bound_variables.push(node.identifier.name.as_str());
        let result = match &node.expression.as_ref() {
            Abstraction(abstraction) => abstraction.accept(self),
            Application(application) => application.accept(self),
        };
        self.bound_variables.pop();
        result
    }

    fn visit_application(
        &mut self,
        node: &'ast ApplicationASTNode,
    ) -> Result<(), SemanticErrors<'ast>> {
        //applications are simple delegations to atomic
        node.left.accept(self)?;
        for i in node.right.iter() {
            i.accept(self)?;
        }
        Ok(())
    }

    fn visit_atomic(&mut self, node: &'ast AtomicASTNode) -> Result<(), SemanticErrors<'ast>> {
        //atomics are also just delegation, either it is an identifier or an expression
        match &node.atom {
            Identifier(identifer) => identifer.accept(self)?,
            AtomicExpression(expression) => expression.accept(self)?,
        }
        Ok(())
    }

    fn visit_identifier(
        &mut self,
        node: &'ast IdentifierASTNode,
    ) -> Result<(), SemanticErrors<'ast>> {
        // identifiers are the most important since all validation happens here
        // this should be obvious since the rest is just delegation
        // but thankfully it is easy
        // kust check if the identifier name exsists in the bound variables or the definitons table

        if !self
            .semantic_table
            .definitions
            .contains_key(node.name.as_str())
            && !self.bound_variables.contains(&node.name.as_str())
        {
            return Err(UndeclaredIdentifier(node.name.as_str()));
        }

        Ok(())
    }
}
