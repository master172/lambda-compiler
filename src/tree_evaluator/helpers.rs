use std::collections::HashSet;

use crate::{
    parser::definitions::{
        Atom::{AtomicExpression, Identifier},
        AtomicASTNode, ExpressionASTNode,
        Expressions::{self, Abstraction, Application},
    },
    semantic_analyser::definitions::SemanticTable,
};

// free variables are variables not bound by any argument in the abstraction
// since only abstractions can bind variables just finding all variables in the AST
// and then removing the ones the abstraction binds is enough
pub fn get_free_variables(node: &ExpressionASTNode, semantics: &SemanticTable) -> HashSet<String> {
    return get_free_inner(&node.expression, semantics);
}

pub fn get_free_inner(expression: &Expressions, semantics: &SemanticTable) -> HashSet<String> {
    match expression {
        Abstraction(abstraction) => {
            let mut variables = get_free_inner(abstraction.expression.as_ref(), &semantics);
            // remove bound variables
            variables.remove(&abstraction.identifier.name);
            variables
        }
        Application(application) => {
            let mut variables = free_variables_atomic(&application.left, &semantics);

            for argument in &application.right {
                variables.extend(free_variables_atomic(argument, &semantics));
            }

            variables
        }
    }
}

pub fn free_variables_atomic(atom: &AtomicASTNode, semantics: &SemanticTable) -> HashSet<String> {
    match &atom.atom {
        Identifier(identifier) => {
            if let Some(definition) = semantics.definitions.get(&identifier.name) {
                get_free_variables(&definition.expression, semantics)
            } else {
                HashSet::from([identifier.name.clone()])
            }
        }
        AtomicExpression(expression) => get_free_variables(expression, semantics),
    }
}

// we need all names in the current scope to find out ones that are not in use
// so that we can generate fresh new names
pub fn get_all_variable_names(node: &ExpressionASTNode) -> HashSet<String> {
    let mut all_names = HashSet::new();
    get_all_names_inner(&node.expression, &mut all_names);
    all_names
}

pub fn get_all_names_inner(expression: &Expressions, hash: &mut HashSet<String>) {
    match expression {
        Abstraction(abstraction) => {
            hash.insert(abstraction.identifier.name.clone());
            get_all_names_inner(&abstraction.expression, hash)
        }
        Application(application) => {
            collect_all_atomic_names(&application.left, hash);

            for arguement in &application.right {
                collect_all_atomic_names(arguement, hash);
            }
        }
    }
}

pub fn collect_all_atomic_names(node: &AtomicASTNode, hash: &mut HashSet<String>) {
    match &node.atom {
        Identifier(identifier) => {
            hash.insert(identifier.name.clone());
        }
        AtomicExpression(expression) => {
            get_all_names_inner(&expression.expression, hash);
        }
    }
}

// we need all variable names and global definiton names to get a new name
// thankfully one function in a struct is enough here
// expression is the one we are replacing with the replacement
// and reserved is used for defintions
pub struct NameGenerator {
    used_names: HashSet<String>,
    counter: usize,
}
impl NameGenerator {
    pub fn new(expression: &ExpressionASTNode, reserved: &SemanticTable) -> Self {
        let mut used = get_all_variable_names(expression);
        for in_use in reserved.definitions.keys() {
            used.insert(in_use.to_string());
        }

        Self {
            used_names: used,
            counter: 0,
        }
    }
    pub fn get_new_name(&mut self, replacement: &AtomicASTNode) -> String {
        self.used_names
            .extend(get_all_atomic_variable_names(replacement));
        loop {
            let candidate = format!("alpha{}", self.counter);
            self.counter += 1;
            if self.used_names.insert(candidate.clone()) {
                return candidate;
            }
        }
    }
}

pub fn get_all_atomic_variable_names(atom: &AtomicASTNode) -> HashSet<String> {
    let mut names = HashSet::new();
    collect_all_atomic_names(atom, &mut names);
    names
}

// renaming starts here, it follows the same pattern of recursive delegation down
// from expression ast node, then abstractions and applications, then to atomics
pub fn replace_in_place(node: &mut ExpressionASTNode, old: &str, new: &str) {
    replace_in_expression(&mut node.expression, old, new);
}

pub fn replace_in_expression(expression: &mut Expressions, old: &str, new: &str) {
    match expression {
        Abstraction(abstraction) => {
            // if an inner abstraction contains the name we are trying to replace them we must stop
            // since otherwise we are changing the name of a variable shadowing the old one
            if &abstraction.identifier.name == old {
                return;
            }

            replace_in_expression(&mut abstraction.expression, old, new);
        }
        Application(application) => {
            replace_in_atomic(&mut application.left, old, new);

            for arguement in &mut application.right {
                replace_in_atomic(arguement, old, new);
            }
        }
    }
}

pub fn replace_in_atomic(node: &mut AtomicASTNode, old: &str, new: &str) {
    match &mut node.atom {
        Identifier(identifier) => {
            if identifier.name == old {
                identifier.name = new.to_owned();
            }
        }
        AtomicExpression(expression) => replace_in_place(expression, old, new),
    }
}

pub fn rename_conflicting_binders(
    expression: &mut Expressions,
    replacement: &AtomicASTNode,
    definitions: &SemanticTable,
    name_generator: &mut NameGenerator,
) {
    match expression {
        Abstraction(abstraction) => {
            let bound = abstraction.identifier.name.clone();
            let replacement_free = free_variables_atomic(replacement, definitions);

            // if any on the free variables in the replacement contains a variable bound by the abstraction
            if replacement_free.contains(&bound) {
                let body = ExpressionASTNode {
                    expression: abstraction.expression.as_ref().clone(),
                };

                let new_name = name_generator.get_new_name(replacement);

                replace_in_expression(&mut abstraction.expression, &bound, &new_name);

                abstraction.identifier.name = new_name;
            }
            rename_conflicting_binders(
                &mut abstraction.expression,
                replacement,
                definitions,
                name_generator,
            );
        }
        Application(application) => {
            rename_conflicting_binders_in_atomic(
                &mut application.left,
                replacement,
                definitions,
                name_generator,
            );
            for argument in &mut application.right {
                rename_conflicting_binders_in_atomic(
                    argument,
                    replacement,
                    definitions,
                    name_generator,
                );
            }
        }
    }
}

pub fn rename_conflicting_binders_in_atomic(
    expression: &mut AtomicASTNode,
    replacement: &AtomicASTNode,
    definitions: &SemanticTable,
    name_generator: &mut NameGenerator,
) {
    match &mut expression.atom {
        Identifier(_) => {}
        AtomicExpression(expression) => rename_conflicting_binders(
            &mut expression.expression,
            replacement,
            definitions,
            name_generator,
        ),
    }
}
