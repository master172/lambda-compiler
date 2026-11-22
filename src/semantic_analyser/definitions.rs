use crate::{
    parser::definitions::{
        AbstractionASTNode, ApplicationASTNode,
        Atom::{AtomicExpression, Identifier},
        AtomicASTNode, DefinitionASTNode, ExpressionASTNode,
        Expressions::{Abstraction, Application},
        IdentifierASTNode, ProgramASTNode, StatementASTNode,
        Statemets::{Definition, ImportStatement, StatementExpression},
    },
    semantic_analyser::definitions::SemanticErrors::{
        AmbiguousIdentifier, DuplicateDefinition, ModuleNotFound, UndeclaredIdentifier,
    },
    visitor::definitons::Visitable,
};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use crate::visitor::definitons::Visitor;

#[derive(Debug)]
pub enum SemanticErrors {
    AmbiguousIdentifier(String),
    UndeclaredIdentifier(String, String),
    DuplicateDefinition(String),
    ModuleNotFound(String),
}

impl Display for SemanticErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UndeclaredIdentifier(name, module) => {
                write!(f, "undeclared identifer: {name} in module {module}")
            }
            DuplicateDefinition(name) => write!(f, "duplicate definiton: {name}"),
            ModuleNotFound(name) => write!(f, "coud not find module {name} for semantic analyis"),
            AmbiguousIdentifier(name) => {
                write!(f, "ambiguous identifier: {name}")
            }
        }
    }
}

#[derive(Debug)]
pub struct SemanticStore {
    pub modules: HashMap<String, SemanticTable>,
    pub global: SemanticTable,

    pub global_imports: HashSet<String>,
    pub module_imports: HashMap<String, HashSet<String>>,
}

impl SemanticStore {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            global: SemanticTable::new(),
            global_imports: HashSet::new(),
            module_imports: HashMap::new(),
        }
    }

    pub fn flatten(self) -> SemanticTable {
        let mut semantic_table: SemanticTable = SemanticTable::new();
        for (name, defintion) in self.global.definitions {
            semantic_table.definitions.insert(name, defintion);
        }
        for (module_name, table) in self.modules {
            for (name, defintion) in table.definitions {
                semantic_table
                    .definitions
                    .insert(format!("{module_name}->{name}"), defintion);
            }
        }

        semantic_table
    }
}
#[derive(Debug)]
pub struct SemanticTable {
    pub definitions: HashMap<String, DefinitionASTNode>,
}

impl SemanticTable {
    pub fn new() -> Self {
        Self {
            definitions: HashMap::new(),
        }
    }
}
pub struct Analyser<'table> {
    semantic_store: &'table SemanticStore,
    bound_variables: Vec<String>,
    current_module: Option<String>,
    allow_free_variables: bool,
}

impl<'table, 'ast> Analyser<'table> {
    pub fn new(store: &'table SemanticStore, allow_free_variables: bool) -> Self {
        Self {
            semantic_store: store,
            bound_variables: vec![],
            current_module: None,
            allow_free_variables,
        }
    }

    fn definition_exists(&self, name: &str) -> Result<bool, SemanticErrors> {
        if name.contains("->") {
            return self.resolve_qualified_identifier(name);
        }

        // determine current scope, resolve scopes table and imports
        let (local_table, import) = match &self.current_module {
            Some(module_name) => {
                let table = self
                    .semantic_store
                    .modules
                    .get(module_name)
                    .ok_or_else(|| ModuleNotFound(module_name.clone()))?;
                (table, self.semantic_store.module_imports.get(module_name))
            }
            None => (
                &self.semantic_store.global,
                Some(&self.semantic_store.global_imports),
            ),
        };
        // check local definitions first
        if local_table.definitions.contains_key(name) {
            return Ok(true);
        }
        // Search all directly imported modules.
        let mut found = false;
        if let Some(imports) = import {
            for imported_module in imports {
                let table = self
                    .semantic_store
                    .modules
                    .get(imported_module)
                    .ok_or_else(|| ModuleNotFound(imported_module.clone()))?;
                if table.definitions.contains_key(name) {
                    if found {
                        return Err(AmbiguousIdentifier(name.to_string()));
                    }
                    found = true;
                }
            }
        }
        Ok(found)
    }

    fn is_module_visible(&self, module_name: &str) -> bool {
        // the module is it self
        if self.current_module.as_deref() == Some(module_name) {
            return true;
        }

        // otherwise find current modules imports and see if that module is imported
        let imports = match &self.current_module {
            Some(module) => self.semantic_store.module_imports.get(module),
            None => Some(&self.semantic_store.global_imports),
        };
        imports.is_some_and(|imports| imports.contains(module_name))
    }

    pub fn resolve_qualified_identifier(&self, identifier: &str) -> Result<bool, SemanticErrors> {
        // if the identifer cannot be split into module_path -> definition then it is not a qualified identifier
        let Some((module_name, definition_name)) = identifier.rsplit_once("->") else {
            return Ok(false);
        };
        // get the table for the module requested
        let table = self
            .semantic_store
            .modules
            .get(module_name)
            .ok_or(SemanticErrors::ModuleNotFound(module_name.to_owned()))?;

        // if it is not imported return false
        if !self.is_module_visible(module_name) {
            return Ok(false);
        }

        // see if the module has the correct definition name
        Ok(table.definitions.contains_key(definition_name))
    }

    pub fn analyse(&mut self, program: &'ast ProgramASTNode) -> Result<(), SemanticErrors> {
        // first analyse every single module
        for module in self.semantic_store.modules.keys() {
            self.current_module = Some(module.clone());

            let table = self
                .semantic_store
                .modules
                .get(module)
                .ok_or_else(|| ModuleNotFound(module.clone()))?;

            for definition in table.definitions.values() {
                definition.accept(self)?;
            }
        }
        self.current_module = None;
        program.accept(self)?;
        Ok(())
    }
}

impl<'table, 'ast> Visitor<'ast, (), SemanticErrors> for Analyser<'table> {
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
        _node: &'ast crate::parser::definitions::ImportASTNode,
    ) -> Result<(), SemanticErrors> {
        // imports are checked a step before we can just pass them
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
        self.bound_variables.push(node.identifier.name.clone());
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
        if !self.definition_exists(&node.name)? && !self.bound_variables.contains(&node.name) {
            let module_name = self
                .current_module
                .clone()
                .unwrap_or(String::from("source"));
            return Err(UndeclaredIdentifier(node.name.to_owned(), module_name));
        }

        Ok(())
    }
}
