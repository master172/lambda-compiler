use std::collections::HashSet;

use crate::{
    parser::{
        definitions::{
            ApplicationASTNode,
            Atom::{AtomicExpression, Identifier},
            AtomicASTNode, ExpressionASTNode,
            Expressions::{self, Abstraction, Application, PhiAbstraction},
            ProgramASTNode,
            Statemets::{Definition, ImportStatement, StatementExpression},
        },
        runtime_abstractions::AbstractionReducable,
    },
    runtime::definitions::Runtime,
    semantic_analyser::definitions::SemanticTable,
    tree_evaluator::helpers::{
        NameGenerator, free_variables_atomic, get_free_variables, rename_conflicting_binders,
        replace_in_expression,
    },
};

pub mod helpers;

// simple enough just normalize all expression
pub fn evaluate<R: Runtime>(
    program: &mut ProgramASTNode,
    definitions: &SemanticTable,
    debug: bool,
    runtime: &mut R,
) {
    for statement in &mut program.statements {
        match &mut statement.statement {
            // no need to normalize a definition
            // definitions do not even exsist in standard lambda calculus
            Definition(_) => {}
            ImportStatement(_) => {}
            // all else is normalized
            StatementExpression(expression) => {
                normalize(expression, &definitions, debug, runtime);
            }
        };
    }
}

// simple enough again call reduce expression until it can no longer be reduced
pub fn normalize<R: Runtime>(
    expression: &mut ExpressionASTNode,
    definitions: &SemanticTable,
    debug: bool,
    runtime: &mut R,
) {
    let mut bound_variables: HashSet<String> = HashSet::new();
    let mut name_generator: NameGenerator = NameGenerator::new(expression, definitions);
    loop {
        if debug {
            println!("{:#?}", expression);
        }
        if reduce_expression_once(
            expression,
            definitions,
            &mut bound_variables,
            &mut name_generator,
            runtime,
        ) == false
        {
            break;
        }
    }
}

// just delegate the work to the respective functions
pub fn reduce_expression_once<R: Runtime>(
    expression: &mut ExpressionASTNode,
    definitions: &SemanticTable,
    bound_variables: &mut HashSet<String>,
    name_generator: &mut NameGenerator,
    runtime: &mut R,
) -> bool {
    let modified: bool;
    match &mut expression.expression {
        Abstraction(abstraction) => {
            modified = reduce_abstraction_once(
                abstraction,
                definitions,
                bound_variables,
                name_generator,
                runtime,
            )
        }
        PhiAbstraction(abstraction) => {
            modified = reduce_abstraction_once(
                abstraction,
                definitions,
                bound_variables,
                name_generator,
                runtime,
            )
        }
        Application(_) => {
            modified = reduce_application_once(
                &mut expression.expression,
                definitions,
                bound_variables,
                name_generator,
                runtime,
            )
        }
    }
    return modified;
}

// just recursively reducing abstractions until all you have is applications
pub fn reduce_abstraction_once<T: AbstractionReducable, R: Runtime>(
    abstraction: &mut T,
    definitions: &SemanticTable,
    bound_variables: &mut HashSet<String>,
    name_generator: &mut NameGenerator,
    runtime: &mut R,
) -> bool {
    // we add to bound variables here which is passed throughout the evaluation
    let name = abstraction.identifier().name.clone();
    let already_present = !bound_variables.insert(name.clone());

    let modified = match abstraction.expression_mut() {
        Abstraction(inner) => {
            reduce_abstraction_once(inner, definitions, bound_variables, name_generator, runtime)
        }
        PhiAbstraction(inner) => {
            reduce_abstraction_once(inner, definitions, bound_variables, name_generator, runtime)
        }
        Application(_) => reduce_application_once(
            abstraction.expression_mut(),
            definitions,
            bound_variables,
            name_generator,
            runtime,
        ),
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
pub fn reduce_application_once<R: Runtime>(
    application: &mut Expressions,
    definitions: &SemanticTable,
    bound_variables: &mut HashSet<String>,
    name_generator: &mut NameGenerator,
    runtime: &mut R,
) -> bool {
    // if this is not an application do not reduce just return false
    let Application(inner_application) = application else {
        return false;
    };

    // first start with reducing the left expression if it is an expression
    let is_abstraction = match &inner_application.left.atom {
        AtomicExpression(expression) => match &expression.expression {
            Abstraction(_) | PhiAbstraction(_) => true,
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
        if reduce_atomic_once(
            &mut inner_application.left,
            definitions,
            bound_variables,
            name_generator,
            runtime,
        ) {
            return true;
        }
    }

    // if it was a function try beta reducing
    // this requires to more if let checks to actually get inner abstraction
    let result = match &inner_application.left.atom {
        AtomicExpression(inner) => match &inner.expression {
            Abstraction(abstraction) => {
                abstraction.beta_reduce(&mut inner_application.right, definitions, name_generator)
            }
            PhiAbstraction(abstraction) => {
                // here we handle the reduction of phi abstractions
                if inner_application.right.is_empty() {
                    None
                } else {
                    // if it could be reduced we just reduce it and return true
                    if reduce_atomic_once(
                        &mut inner_application.right[0],
                        definitions,
                        bound_variables,
                        name_generator,
                        runtime,
                    ) {
                        return true;
                    }
                    // otherwise tell the runtime to do something with it, and then beta reduce it
                    runtime
                        .phi(
                            abstraction.communication_stream_id,
                            &inner_application.right[0],
                        )
                        .unwrap();
                    abstraction.beta_reduce(
                        &mut inner_application.right,
                        definitions,
                        name_generator,
                    )
                }
            }
            _ => None,
        },
        _ => None,
    };

    if let Some(result) = result {
        *application = result;
        return true;
    }
    // reduce right arguements if no beta substitution happens
    for arguement in &mut inner_application.right {
        if reduce_atomic_once(
            arguement,
            definitions,
            bound_variables,
            name_generator,
            runtime,
        ) {
            return true;
        }
    }
    false
}

pub fn beta_reduce<T: AbstractionReducable>(
    abstraction: &T,
    application: &mut ApplicationASTNode,
    definitions: &SemanticTable,
    name_generator: &mut NameGenerator,
) -> Option<Expressions> {
    let target = abstraction.identifier().name.clone();
    // need to get the inner application this way
    let argument = application.right.remove(0);

    let remaining_arguments = std::mem::take(&mut application.right);

    let mut body = abstraction.expression().clone();

    substitute_expression(&mut body, &argument, definitions, &target, name_generator);

    Some(if remaining_arguments.is_empty() {
        body
    } else {
        Application(ApplicationASTNode {
            left: Box::new(AtomicASTNode {
                atom: AtomicExpression(ExpressionASTNode { expression: body }),
            }),
            right: remaining_arguments,
        })
    })
}
// this is mostly work delegation
pub fn reduce_atomic_once<R: Runtime>(
    atomic: &mut AtomicASTNode,
    definitions: &SemanticTable,
    bound_variables: &mut HashSet<String>,
    name_generator: &mut NameGenerator,
    runtime: &mut R,
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
            modified = reduce_expression_once(
                expression,
                definitions,
                bound_variables,
                name_generator,
                runtime,
            )
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
    name_generator: &mut NameGenerator,
) {
    match expression {
        Abstraction(abstraction) => {
            substitute_abstraction(
                abstraction,
                replacement,
                definitions,
                target,
                name_generator,
            );
        }
        // hopefully there is a better way to do this
        PhiAbstraction(abstraction) => {
            substitute_abstraction(
                abstraction,
                replacement,
                definitions,
                target,
                name_generator,
            );
        }
        Application(application) => {
            substitute_atomic(
                &mut application.left,
                replacement,
                definitions,
                target,
                name_generator,
            );

            for arguement in &mut application.right {
                substitute_atomic(arguement, replacement, definitions, target, name_generator);
            }
        }
    }
}

pub fn substitute_abstraction<T: AbstractionReducable>(
    abstraction: &mut T,
    replacement: &AtomicASTNode,
    definitions: &SemanticTable,
    target: &str,
    name_generator: &mut NameGenerator,
) {
    // variable shadowed do not try to substitute further
    let bound = abstraction.identifier().name.clone();
    if bound == target {
        return;
    }

    // this also handles alpha renaming
    // first if the variable is bound no change occurs
    let body = ExpressionASTNode {
        expression: abstraction.expression().clone(),
    };

    if !get_free_variables(&body, &definitions).contains(target) {
        return;
    }

    // see which variables can be replaced in the replacement
    let replacement_free = free_variables_atomic(replacement, &definitions);

    // if substituting the replacement would cause one of its free variables to become bound
    // we alpha rename it first
    if replacement_free.contains(&bound) {
        let new_name = name_generator.get_new_name(replacement);
        replace_in_expression(abstraction.expression_mut(), &bound, &new_name);
        abstraction.identifier_mut().name = new_name;
    }

    substitute_expression(
        abstraction.expression_mut(),
        replacement,
        definitions,
        target,
        name_generator,
    );
}
// replace atomic identifiers with their expressions
// otherwise keep replacing the expressions with targets
pub fn substitute_atomic(
    expression: &mut AtomicASTNode,
    replacement: &AtomicASTNode,
    definitions: &SemanticTable,
    target: &str,
    name_generator: &mut NameGenerator,
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

                rename_conflicting_binders(
                    &mut expanded_defintion.expression,
                    replacement,
                    definitions,
                    name_generator,
                );
                substitute_expression(
                    &mut expanded_defintion.expression,
                    replacement,
                    definitions,
                    target,
                    name_generator,
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
                name_generator,
            );
        }
    }
}
