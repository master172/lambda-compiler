use std::collections::HashMap;

use crate::{
    compiler_definitions::ModuleStore,
    parser::definitions::{ProgramASTNode, Statemets::Definition},
    semantic_analyser::definitions::{
        SemanticErrors::{self, DuplicateDefinition},
        SemanticStore, SemanticTable,
    },
};

pub mod definitions;

//no need for a tree evaluator here since definitons are statements
pub fn create_semantic_table<'a, 'm>(
    module_store: &'m mut ModuleStore,
    topological_dependency_graph: &Vec<String>,
    program: &'a ProgramASTNode,
) -> Result<SemanticStore, SemanticErrors> {
    let mut semantic_store: SemanticStore = SemanticStore {
        modules: HashMap::new(),
        global: SemanticTable {
            definitions: HashMap::new(),
        },
    };
    for key in topological_dependency_graph {
        let module = module_store
            .modules
            .get(key)
            .ok_or(SemanticErrors::ModuleNotFound(key.to_owned()))?;
        // initalize an empty semantic table
        semantic_store.modules.insert(
            key.clone(),
            SemanticTable {
                definitions: HashMap::new(),
            },
        );
        for statement in module.statements.iter() {
            match &statement.statement {
                Definition(definition) => {
                    if semantic_store
                        .modules
                        .get(key)
                        .ok_or(SemanticErrors::ModuleNotFound(key.to_owned()))?
                        .definitions
                        .contains_key(&definition.identifier.name)
                    {
                        return Err(DuplicateDefinition(definition.identifier.name.to_owned()));
                    }
                    semantic_store
                        .modules
                        .get_mut(key)
                        .ok_or(SemanticErrors::ModuleNotFound(key.to_owned()))?
                        .definitions
                        .insert(definition.identifier.name.clone(), definition.clone());
                }
                _ => {}
            }
        }
    }

    for statement in program.statements.iter() {
        match &statement.statement {
            Definition(definition) => {
                if semantic_store
                    .global
                    .definitions
                    .contains_key(&definition.identifier.name)
                {
                    return Err(DuplicateDefinition(definition.identifier.name.to_owned()));
                }
                semantic_store
                    .global
                    .definitions
                    .insert(definition.identifier.name.clone(), definition.clone());
            }
            _ => {}
        }
    }

    return Ok(semantic_store);
}
