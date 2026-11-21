use std::{
    collections::{HashMap, HashSet},
    env,
    path::PathBuf,
};

use crate::{
    compiler_definitions::ModuleStore,
    dependency_graph::definitions::{DependencyErrors, DependencyGraph, Module},
    parser::definitions::{ProgramASTNode, Statemets::ImportStatement},
};

mod definitions;

pub fn create_dependency_graph(
    ast: &ProgramASTNode,
    main_identifier: &str,
    module_store: &ModuleStore,
) -> Result<DependencyGraph, DependencyErrors> {
    let mut graph: DependencyGraph = DependencyGraph {
        modules: HashMap::new(),
    };
    let formatted_main_identifier = normalize_identifier(main_identifier)?;
    // pass 1 add the main source file
    graph.modules.insert(
        formatted_main_identifier.clone(),
        Module {
            name: formatted_main_identifier.clone(),
            imports: HashSet::new(),
        },
    );
    // pass 1 for all includes
    for (identifier, _ast) in &module_store.modules {
        graph.modules.insert(
            normalize_identifier(identifier)?,
            Module {
                name: normalize_identifier(identifier)?,
                imports: HashSet::new(),
            },
        );
    }
    // adding dependencies
    // first for the main source file
    for statement in &ast.statements {
        match &statement.statement {
            ImportStatement(import) => {
                let identifier = &import.identifier.name;
                if let Some(module) = graph.modules.get_mut(&formatted_main_identifier) {
                    let formatted_identifier = identifier.to_string();
                    module.imports.insert(formatted_identifier);
                } else {
                    return Err(DependencyErrors::ModuleNotProvided(identifier.clone()));
                }
            }
            _ => {}
        }
    }
    // then for all other ast's
    for (module_identifier, including_ast) in &module_store.modules {
        let modified_main_identifier = normalize_identifier(module_identifier)?;
        for statement in &including_ast.statements {
            match &statement.statement {
                ImportStatement(import) => {
                    let identifier = &import.identifier.name;
                    if let Some(module) = graph.modules.get_mut(&modified_main_identifier) {
                        module.imports.insert(identifier.to_string());
                    } else {
                        return Err(DependencyErrors::ModuleNotProvided(
                            modified_main_identifier,
                        ));
                    }
                }
                _ => {}
            }
        }
    }

    //dependency graph is ready
    Ok(graph)
}

fn normalize_identifier(identifier: &str) -> Result<String, DependencyErrors> {
    let root = env::current_dir().map_err(|_| DependencyErrors::NoDirectory)?;
    let path = PathBuf::from(identifier);
    let relative = path
        .strip_prefix(&root)
        .map_err(|_| DependencyErrors::PathConversionError(identifier.to_string()))?;
    let relative = relative.with_extension("");

    Ok(relative
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("->"))
}
