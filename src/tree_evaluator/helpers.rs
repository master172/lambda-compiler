use std::collections::HashSet;

use crate::parser::definitions::{
    Atom::{AtomicExpression, Identifier},
    AtomicASTNode, ExpressionASTNode,
    Expressions::{self},
};

pub fn get_free_variables(node: &ExpressionASTNode) -> HashSet<String> {
    return get_free_inner(&node.expression);
}

// free variables are variables not bound by any argument in the abstraction
// since only abstractions can bind variables just finding all variables in the AST
// and then removing the ones the abstraction binds is enough
pub fn get_free_inner(expression: &Expressions) -> HashSet<String> {
    match expression {
        Expressions::Abstraction(abstraction) => {
            let mut variables = get_free_inner(abstraction.expression.as_ref());
            // remove bound variables
            variables.remove(&abstraction.identifier.name);
            variables
        }
        Expressions::Application(application) => {
            let mut variables = free_variables_atomic(&application.left);

            for argument in &application.right {
                variables.extend(free_variables_atomic(argument));
            }

            variables
        }
    }
}

pub fn free_variables_atomic(atom: &AtomicASTNode) -> HashSet<String> {
    match &atom.atom {
        Identifier(identifier) => HashSet::from([identifier.name.clone()]),
        AtomicExpression(expression) => get_free_variables(expression),
    }
}
