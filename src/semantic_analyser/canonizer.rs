use crate::{
    parser::definitions::{
        Atom::{AtomicExpression, Identifier},
        Expressions::{Abstraction, Application},
        ProgramASTNode,
        Statemets::{Definition, ImportStatement, StatementExpression},
    },
    semantic_analyser::{
        canonizer::definitions::CanoniclaizationContext,
        definitions::{
            SemanticErrors::{self, ModuleNotFound},
            SemanticStore,
        },
    },
    visitor::definitons::{MutableVisitor, Visitable},
};

pub mod definitions;

pub struct Canonicalizer {
    context: CanoniclaizationContext,
    bound_variables: Vec<String>,
    current_module: Option<String>,
}

impl Canonicalizer {
    pub fn new(context: CanoniclaizationContext) -> Self {
        Self {
            context,
            bound_variables: Vec::new(),
            current_module: None,
        }
    }

    pub fn canonicalize(
        &mut self,
        store: &mut SemanticStore,
        program: &mut ProgramASTNode,
    ) -> Result<(), SemanticErrors> {
        let modules: Vec<String> = store.modules.keys().cloned().collect();
        for module in modules {
            self.current_module = Some(module.clone());

            let table = store
                .modules
                .get_mut(&module)
                .ok_or_else(|| ModuleNotFound(module.clone()))?;

            for definition in table.definitions.values_mut() {
                definition.accept_mut(self)?;
            }
        }
        // Canonicalize global definitions
        self.current_module = None;

        for definition in store.global.definitions.values_mut() {
            definition.accept_mut(self)?;
        }

        // Canonicalize source program
        program.accept_mut(self)?;

        Ok(())
    }
}

impl<'ast> MutableVisitor<'ast, (), SemanticErrors> for Canonicalizer {
    fn visit_program(&mut self, node: &'ast mut ProgramASTNode) -> Result<(), SemanticErrors> {
        for statement in node.statements.iter_mut() {
            statement.accept_mut(self)?;
        }
        Ok(())
    }
    fn visit_statement(
        &mut self,
        node: &'ast mut crate::parser::definitions::StatementASTNode,
    ) -> Result<(), SemanticErrors> {
        match &mut node.statement {
            Definition(definition) => definition.accept_mut(self)?,
            StatementExpression(expression) => expression.accept_mut(self)?,
            ImportStatement(import) => import.accept_mut(self)?,
        }

        Ok(())
    }

    fn visit_import(
        &mut self,
        _node: &'ast mut crate::parser::definitions::ImportASTNode,
    ) -> Result<(), SemanticErrors> {
        Ok(())
    }

    fn visit_definition(
        &mut self,
        node: &'ast mut crate::parser::definitions::DefinitionASTNode,
    ) -> Result<(), SemanticErrors> {
        node.expression.accept_mut(self)
    }

    fn visit_expression(
        &mut self,
        node: &'ast mut crate::parser::definitions::ExpressionASTNode,
    ) -> Result<(), SemanticErrors> {
        match &mut node.expression {
            Abstraction(abstraction) => abstraction.accept_mut(self)?,
            Application(application) => application.accept_mut(self)?,
        }
        Ok(())
    }

    fn visit_abstraction(
        &mut self,
        node: &'ast mut crate::parser::definitions::AbstractionASTNode,
    ) -> Result<(), SemanticErrors> {
        self.bound_variables.push(node.identifier.name.clone());

        let result = match node.expression.as_mut() {
            Abstraction(abstraction) => abstraction.accept_mut(self),
            Application(application) => application.accept_mut(self),
        };

        self.bound_variables.pop();
        result
    }

    fn visit_application(
        &mut self,
        node: &'ast mut crate::parser::definitions::ApplicationASTNode,
    ) -> Result<(), SemanticErrors> {
        node.left.accept_mut(self)?;

        for atom in node.right.iter_mut() {
            atom.accept_mut(self)?;
        }
        Ok(())
    }

    fn visit_atomic(
        &mut self,
        node: &'ast mut crate::parser::definitions::AtomicASTNode,
    ) -> Result<(), SemanticErrors> {
        match &mut node.atom {
            Identifier(identifier) => identifier.accept_mut(self)?,
            AtomicExpression(expression) => expression.accept_mut(self)?,
        }

        Ok(())
    }

    fn visit_identifier(
        &mut self,
        node: &'ast mut crate::parser::definitions::IdentifierASTNode,
    ) -> Result<(), SemanticErrors> {
        // Bound variables are never canonicalized
        if self.bound_variables.contains(&node.name) {
            return Ok(());
        }
        // Resolve the identifier using the current module's scope
        if let Some(canonical_name) = self
            .context
            .resolve_identifier(&node.name, self.current_module.as_deref())
        {
            node.name = canonical_name;
        }

        // Unresolved identifiers are left unchanged.
        // Semantic analysis has already validated them.
        Ok(())
    }
}
