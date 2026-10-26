use crate::{
    parser::definitions::{
        Atom::{AtomicExpression, Identifier},
        Expressions::{Abstraction, Application},
        Statemets::{Definition, StatementExpression},
    },
    visitor::definitons::{Visitable, Visitor},
};

pub struct PrettyPrinter {}

impl<'ast> Visitor<'ast, String, ()> for PrettyPrinter {
    // verified this does its work for each statement
    fn visit_program(
        &mut self,
        node: &'ast crate::parser::definitions::ProgramASTNode,
    ) -> Result<String, ()> {
        let mut resultant: String = String::new();
        for i in &node.statements {
            resultant += &i.accept(self)?;
            resultant += "\n";
        }
        Ok(resultant)
    }

    // just delegation and addition expected
    fn visit_statement(
        &mut self,
        node: &'ast crate::parser::definitions::StatementASTNode,
    ) -> Result<String, ()> {
        let mut resultant: String = String::new();
        match &node.statement {
            Definition(definition) => resultant += &definition.accept(self)?,
            StatementExpression(expression) => resultant += &expression.accept(self)?,
        }
        Ok(resultant)
    }

    // more delegation and addition but it is verified
    fn visit_definition(
        &mut self,
        node: &'ast crate::parser::definitions::DefinitionASTNode,
    ) -> Result<String, ()> {
        let mut resultant: String = String::new();
        let expression = &node.expression;
        resultant += &expression.accept(self)?;
        Ok(format!("{} = {}", node.identifier.name, resultant))
    }

    // still just delegation and addition
    // this is more true since an expression is just a substitute for two kinds of statements
    fn visit_expression(
        &mut self,
        node: &'ast crate::parser::definitions::ExpressionASTNode,
    ) -> Result<String, ()> {
        let mut resultant: String = String::new();
        match &node.expression {
            Abstraction(abstraction) => resultant += &abstraction.accept(self)?,
            Application(application) => resultant += &application.accept(self)?,
        }
        Ok(resultant)
    }

    fn visit_abstraction(
        &mut self,
        node: &'ast crate::parser::definitions::AbstractionASTNode,
    ) -> Result<String, ()> {
        let mut resultant: String = String::new();
        match &node.expression.as_ref() {
            Abstraction(abstraction) => resultant += &abstraction.accept(self)?,
            Application(application) => resultant += &application.accept(self)?,
        };
        Ok(format!("λ{}.{resultant}", node.identifier.name))
    }

    fn visit_application(
        &mut self,
        node: &'ast crate::parser::definitions::ApplicationASTNode,
    ) -> Result<String, ()> {
        let mut resultant: String;
        let len = node.right.len();
        if len == 0 {
            resultant = format!("{}", &node.left.accept(self)?);
        } else {
            resultant = format!("{} ", &node.left.accept(self)?);
        }

        for (index, item) in node.right.iter().enumerate() {
            if index == len - 1 {
                resultant += &format!("{}", &item.accept(self)?);
            } else {
                resultant += &format!("{} ", &item.accept(self)?);
            }
        }
        Ok(resultant)
    }

    // visit atomic is also just a simple delegator and adder
    fn visit_atomic(
        &mut self,
        node: &'ast crate::parser::definitions::AtomicASTNode,
    ) -> Result<String, ()> {
        let mut resultant: String = String::new();
        match &node.atom {
            Identifier(identifer) => resultant += &identifer.accept(self)?,
            AtomicExpression(expression) => {
                resultant += &format!("({})", &expression.accept(self)?)
            }
        }
        Ok(resultant)
    }

    fn visit_identifier(
        &mut self,
        node: &'ast crate::parser::definitions::IdentifierASTNode,
    ) -> Result<String, ()> {
        Ok(format!("{}", node.name))
    }
}
