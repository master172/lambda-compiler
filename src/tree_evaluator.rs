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
pub fn evaluate(program: &mut ProgramASTNode, definitions: &SemanticTable<'_>) {
    for statement in &mut program.statements {
        match &mut statement.statement {
            // no need to normalize a definition
            // definitions do not even exsist in standard lambda calculus
            Definition(_) => {}
            // all else is normalized
            StatementExpression(expression) => normalize(expression, definitions),
        };
    }
}

// simple enough again call reduce expression until it can no longer be reduced
pub fn normalize(expression: &mut ExpressionASTNode, definitions: &SemanticTable<'_>) {
    loop {
        if reduce_expression_once(expression, definitions) == false {
            break;
        }
    }
}

// just delegate the work to the respective functions
pub fn reduce_expression_once(
    expression: &mut ExpressionASTNode,
    definitions: &SemanticTable<'_>,
) -> bool {
    let modified: bool;
    match &mut expression.expression {
        Abstraction(abstraction) => modified = reduce_abstraction_once(abstraction, definitions),
        Application(_) => {
            modified = reduce_application_once(&mut expression.expression, definitions)
        }
    }
    return modified;
}

// just recursively reducing abstractions until all you have is applications
pub fn reduce_abstraction_once(
    abstraction: &mut AbstractionASTNode,
    definitions: &SemanticTable<'_>,
) -> bool {
    match abstraction.expression.as_mut() {
        Abstraction(abstraction) => reduce_abstraction_once(abstraction, definitions),
        Application(_) => reduce_application_once(&mut abstraction.expression, definitions),
    }
}

// probbably the most important function since beta reduction happens here
// and we check for alpha renaming here as well
pub fn reduce_application_once(
    application: &mut Expressions,
    definitions: &SemanticTable<'_>,
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

    // if it is not an abstraction try reducing the identifier once if it reduces return true
    if !is_abstraction {
        if reduce_atomic_once(&mut inner_application.left, definitions) {
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

                substitute_expression(&mut body, &arguement, definitions, &target);

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
        if reduce_atomic_once(arguement, definitions) {
            return true;
        }
    }
    false
}

// this is mostly work delegation
pub fn reduce_atomic_once(atomic: &mut AtomicASTNode, definitions: &SemanticTable<'_>) -> bool {
    let modified: bool;
    match &mut atomic.atom {
        Identifier(identifier) => {
            if let Some(definition) = definitions.definitions.get(identifier.name.as_str()) {
                atomic.atom = AtomicExpression(definition.expression.clone());
                modified = true;
            } else {
                modified = false;
            }
        }
        AtomicExpression(expression) => modified = reduce_expression_once(expression, definitions),
    }
    modified
}

// this is just mostly reduce until you can delegate to atomics
pub fn substitute_expression(
    expression: &mut Expressions,
    replacement: &AtomicASTNode,
    definitions: &SemanticTable<'_>,
    target: &str,
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

            if !get_free_variables(&body).contains(target) {
                return;
            }

            // see which variables can be replaced in the replacement
            let replacement_free = free_variables_atomic(replacement);

            // if substituting the replacement would cause one of its free variables to become bound
            // we alpha rename it first
            if replacement_free.contains(&bound) {
                let new_name = get_new_name(&body, replacement, definitions);
                replace_in_expression(abstraction.expression.as_mut(), &bound, &new_name);
                abstraction.identifier.name = new_name;
            }

            substitute_expression(
                abstraction.expression.as_mut(),
                replacement,
                definitions,
                target,
            );
        }
        Application(application) => {
            substitute_atomic(&mut application.left, replacement, definitions, target);

            for arguement in &mut application.right {
                substitute_atomic(arguement, replacement, definitions, target);
            }
        }
    }
}

// replace atomic identifiers with their expressions
// otherwise keep replacing the expressions with targets
pub fn substitute_atomic(
    expression: &mut AtomicASTNode,
    replacement: &AtomicASTNode,
    definitions: &SemanticTable<'_>,
    target: &str,
) {
    match &mut expression.atom {
        Identifier(identifier) => {
            if identifier.name == target {
                *expression = replacement.clone();
            }
        }
        AtomicExpression(inner) => {
            substitute_expression(&mut inner.expression, replacement, definitions, target);
        }
    }
}
