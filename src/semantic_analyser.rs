use std::collections::{HashMap, HashSet};

use crate::{
    compiler_definitions::ModuleStore,
    parser::definitions::{
        ProgramASTNode,
        Statemets::{Definition, ImportStatement},
    },
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
    let mut semantic_store: SemanticStore = SemanticStore::new();
    for key in topological_dependency_graph {
        let module = module_store
            .modules
            .get(key)
            .ok_or(SemanticErrors::ModuleNotFound(key.to_owned()))?;
        // initalize an empty semantic table
        let mut table = SemanticTable {
            definitions: HashMap::new(),
        };
        let mut imports: HashSet<String> = HashSet::new();
        for statement in module.statements.iter() {
            match &statement.statement {
                Definition(definition) => {
                    let name = &definition.identifier.name;
                    if table.definitions.contains_key(name) {
                        return Err(DuplicateDefinition(name.clone()));
                    }
                    table.definitions.insert(name.clone(), definition.clone());
                }
                ImportStatement(import) => {
                    imports.insert(import.identifier.name.clone());
                }
                _ => {}
            }
        }
        semantic_store.modules.insert(key.clone(), table);
        semantic_store.module_imports.insert(key.clone(), imports);
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
            ImportStatement(import) => {
                semantic_store
                    .global_imports
                    .insert(import.identifier.name.clone());
            }
            _ => {}
        }
    }

    // validation of imported modules
    // dependencies firsy
    for imports in semantic_store.module_imports.values() {
        for import in imports {
            if !semantic_store.modules.contains_key(import) {
                return Err(SemanticErrors::ModuleNotFound(import.clone()));
            }
        }
    }
    // then the source file
    for import in &semantic_store.global_imports {
        if !semantic_store.modules.contains_key(import) {
            return Err(SemanticErrors::ModuleNotFound(import.clone()));
        }
    }
    return Ok(semantic_store);
}
