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
        Application(application) => modified = reduce_application_once(application, definitions),
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
        Application(application) => reduce_application_once(application, definitions),
    }
}

pub fn reduce_application_once(
    application: &mut ApplicationASTNode,
    definitions: &SemanticTable<'_>,
) -> bool {
    let modified: bool = false;
    if reduce_atomic_once(&mut application.left, definitions) {
        return true;
    }
    //TODO: see if the left and right[0] can be beta substituted and if not then recursively reduce right atomics
    return modified;
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
    replacement: &ExpressionASTNode,
    target: &str,
) {
    match expression {
        // variable shadowed do not try to substitute further
        Abstraction(abstraction) => {
            if abstraction.identifier.name == target {
                return;
            }

            substitute_expression(abstraction.expression.as_mut(), replacement, target);
        }
        Application(application) => {
            substitute_atomic(&mut application.left, replacement, target);

            for arguement in &mut application.right {
                substitute_atomic(arguement, replacement, target);
            }
        }
    }
}

// replace atomic identifiers with their expressions
// otherwise keep replacing the expressions with targets
pub fn substitute_atomic(
    expression: &mut AtomicASTNode,
    replacement: &ExpressionASTNode,
    target: &str,
) {
    match &mut expression.atom {
        Identifier(identifier) => {
            if identifier.name == target {
                expression.atom = AtomicExpression(replacement.clone());
            }
        }
        AtomicExpression(inner) => {
            substitute_expression(&mut inner.expression, replacement, target);
        }
    }
}
