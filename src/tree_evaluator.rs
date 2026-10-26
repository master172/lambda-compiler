use std::collections::HashSet;

use crate::{
    parser::definitions::{
        AbstractionASTNode, ApplicationASTNode,
        Atom::{AtomicExpression, Identifier},
        AtomicASTNode, ExpressionASTNode,
        Expressions::{self, Abstraction, Application},
        ProgramASTNode,
        Statemets::{Definition, StatementExpression},
    },
    semantic_analyser::definitions::SemanticTable,
    tree_evaluator::helpers::{
        free_variables_atomic, get_free_variables, get_new_name, replace_in_expression,
    },
};

mod helpers;

// simple enough just normalize all expression
pub fn evaluate(program: &mut ProgramASTNode, definitions: &SemanticTable, debug: bool) {
    for statement in &mut program.statements {
        match &mut statement.statement {
            // no need to normalize a definition
            // definitions do not even exsist in standard lambda calculus
            Definition(_) => {}
            // all else is normalized
            StatementExpression(expression) => {
                normalize(expression, definitions, debug);
            }
        };
    }
}

// simple enough again call reduce expression until it can no longer be reduced
pub fn normalize(expression: &mut ExpressionASTNode, definitions: &SemanticTable, debug: bool) {
    let mut bound_variables: HashSet<String> = HashSet::new();
    loop {
        if debug {
            println!("{:#?}", expression);
        }
        if reduce_expression_once(expression, definitions, &mut bound_variables) == false {
            break;
        }
    }
}

// just delegate the work to the respective functions
pub fn reduce_expression_once(
    expression: &mut ExpressionASTNode,
    definitions: &SemanticTable,
    bound_variables: &mut HashSet<String>,
) -> bool {
    let modified: bool;
    match &mut expression.expression {
        Abstraction(abstraction) => {
            modified = reduce_abstraction_once(abstraction, definitions, bound_variables)
        }
        Application(_) => {
            modified =
                reduce_application_once(&mut expression.expression, definitions, bound_variables)
        }
    }
    return modified;
}

// just recursively reducing abstractions until all you have is applications
pub fn reduce_abstraction_once(
    abstraction: &mut AbstractionASTNode,
    definitions: &SemanticTable,
    bound_variables: &mut HashSet<String>,
) -> bool {
    // we add to bound variables here which is passed throughout the evaluation
    let name = abstraction.identifier.name.clone();
    let already_present = !bound_variables.insert(name.clone());

    let modified = match abstraction.expression.as_mut() {
        Abstraction(inner) => reduce_abstraction_once(inner, definitions, bound_variables),
        Application(_) => {
            reduce_application_once(&mut abstraction.expression, definitions, bound_variables)
        }
    };

    // we only remove the bound variable if it was not already present during the recursion
    // this prevents removing bound variables that were nested
    if !already_present {
        bound_variables.remove(&name);
    }
    modified
}

// probbably the most important function since beta reduction happens here
// and we check for alpha renaming here as well
pub fn reduce_application_once(
    application: &mut Expressions,
    definitions: &SemanticTable,
    bound_variables: &mut HashSet<String>,
) -> bool {
    // if this is not an application do not reduce just return false
    let Application(inner_application) = application else {
        return false;
    };

    // first start with reducing the left expression if it is an expression
    let is_abstraction = match &inner_application.left.atom {
        AtomicExpression(expression) => match &expression.expression {
            Abstraction(_) => true,
            _ => false,
        },
        _ => false,
    };

    if inner_application.right.is_empty() {
        if let AtomicExpression(inner) = &inner_application.left.atom {
            *application = inner.expression.clone();
            return true;
        }
    }
    // if it is not an abstraction try reducing the identifier once if it reduces return true
    if !is_abstraction {
        if reduce_atomic_once(&mut inner_application.left, definitions, bound_variables) {
            return true;
        }
    }

    // if it was a function try beta reducing
    // this requires to more if let checks to actually get inner abstraction
    if let AtomicExpression(inner) = &inner_application.left.atom {
        if let Abstraction(abstraction) = &inner.expression {
            if !inner_application.right.is_empty() {
                let target = abstraction.identifier.name.clone();
                let arguement = inner_application.right.remove(0);

                let remaining_arguements = std::mem::take(&mut inner_application.right);
                let mut body = abstraction.expression.as_ref().clone();

                substitute_expression(&mut body, &arguement, definitions, &target, bound_variables);

                if remaining_arguements.is_empty() {
                    *application = body;
                } else {
                    *application = Application(ApplicationASTNode {
                        left: Box::new(AtomicASTNode {
                            atom: AtomicExpression(ExpressionASTNode { expression: body }),
                        }),
                        right: remaining_arguements,
                    })
                }

                return true;
            }
        }
    }

    // reduce right arguements if no beta substitution happens
    for arguement in &mut inner_application.right {
        if reduce_atomic_once(arguement, definitions, bound_variables) {
            return true;
        }
    }
    false
}

// this is mostly work delegation
pub fn reduce_atomic_once(
    atomic: &mut AtomicASTNode,
    definitions: &SemanticTable,
    bound_variables: &mut HashSet<String>,
) -> bool {
    let modified: bool;
    match &mut atomic.atom {
        Identifier(identifier) => {
            // we use bound variables here to check wether a name is bound to an abstraction
            if let Some(definition) = definitions.definitions.get(identifier.name.as_str())
                && !bound_variables.contains(&identifier.name)
            {
                atomic.atom = AtomicExpression(definition.expression.clone());
                modified = true;
            } else {
                modified = false;
            }
        }
        AtomicExpression(expression) => {
            modified = reduce_expression_once(expression, definitions, bound_variables)
        }
    }
    modified
}

// this is just mostly reduce until you can delegate to atomics
pub fn substitute_expression(
    expression: &mut Expressions,
    replacement: &AtomicASTNode,
    definitions: &SemanticTable,
    target: &str,
    bound_variables: &mut HashSet<String>,
) {
    match expression {
        Abstraction(abstraction) => {
            // variable shadowed do not try to substitute further
            let bound = abstraction.identifier.name.clone();
            if bound == target {
                return;
            }

            // this also handles alpha renaming
            // first if the variable is bound no change occurs
            let body = ExpressionASTNode {
                expression: abstraction.expression.as_ref().clone(),
            };

            if !get_free_variables(&body, &definitions).contains(target) {
                return;
            }

            // see which variables can be replaced in the replacement
            let replacement_free = free_variables_atomic(replacement, &definitions);

            // if substituting the replacement would cause one of its free variables to become bound
            // we alpha rename it first
            if replacement_free.contains(&bound) {
                let new_name = get_new_name(&body, replacement, definitions, bound_variables);
                replace_in_expression(abstraction.expression.as_mut(), &bound, &new_name);
                abstraction.identifier.name = new_name;
            }

            let already_present = !bound_variables.insert(abstraction.identifier.name.clone());

            substitute_expression(
                abstraction.expression.as_mut(),
                replacement,
                definitions,
                target,
                bound_variables,
            );

            if !already_present {
                bound_variables.remove(&abstraction.identifier.name);
            }
        }
        Application(application) => {
            substitute_atomic(
                &mut application.left,
                replacement,
                definitions,
                target,
                bound_variables,
            );

            for arguement in &mut application.right {
                substitute_atomic(arguement, replacement, definitions, target, bound_variables);
            }
        }
    }
}

// replace atomic identifiers with their expressions
// otherwise keep replacing the expressions with targets
pub fn substitute_atomic(
    expression: &mut AtomicASTNode,
    replacement: &AtomicASTNode,
    definitions: &SemanticTable,
    target: &str,
    bound_variables: &mut HashSet<String>,
) {
    match &mut expression.atom {
        Identifier(identifier) => {
            if identifier.name == target {
                *expression = replacement.clone();
                return;
            }

            // we also need to check for expansions in globally defined definitions
            // so we substitute throughout its body as well
            if let Some(definition) = definitions.definitions.get(identifier.name.as_str()) {
                let mut expanded_defintion = definition.expression.clone();

                substitute_expression(
                    &mut expanded_defintion.expression,
                    replacement,
                    definitions,
                    target,
                    bound_variables,
                );

                *expression = AtomicASTNode {
                    atom: AtomicExpression(expanded_defintion),
                }
            }
        }
        AtomicExpression(inner) => {
            substitute_expression(
                &mut inner.expression,
                replacement,
                definitions,
                target,
                bound_variables,
            );
        }
    }
}
