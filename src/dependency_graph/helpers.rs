use std::{collections::HashMap, fmt};

use crate::dependency_graph::{
    definitions::{DependencyGraph, Module},
    helpers::DFSErrors::{CycleDetected, DependnecyNotFound},
};

#[derive(PartialEq, Eq)]
pub enum VisitStates {
    Visited,
    Visiting,
    UnVisisted,
}

pub enum DFSErrors {
    CycleDetected((String, String)),
    DependnecyNotFound(String),
}

impl fmt::Display for DFSErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            CycleDetected((mod1, mod2)) => {
                write!(f, "cycle detected in dependencies {}, {}", mod1, mod2)
            }
            DependnecyNotFound(name) => write!(f, "dependnecy not found {}", name),
        }
    }
}
pub fn depth_first_search(graph: &DependencyGraph) -> Result<(), DFSErrors> {
    let mut visit_map: HashMap<String, VisitStates> = HashMap::new();
    // first mark all as unvisited
    for (name, _module) in &graph.modules {
        visit_map.insert(name.to_owned(), VisitStates::UnVisisted);
    }
    // now recursively iterate over the list
    for (name, module) in &graph.modules {
        let state = visit_map
            .get(name)
            .ok_or(DependnecyNotFound(name.clone()))?;
        if state == &VisitStates::UnVisisted {
            visit_map.insert(name.to_owned(), VisitStates::Visiting);
            dfs_helper(graph, module, &mut visit_map)?;
            visit_map.insert(name.to_owned(), VisitStates::Visited);
        }
    }

    Ok(())
}

pub fn dfs_helper(
    graph: &DependencyGraph,
    module: &Module,
    visit_map: &mut HashMap<String, VisitStates>,
) -> Result<(), DFSErrors> {
    for name in &module.imports {
        let visited = visit_map
            .get(name)
            .ok_or(DependnecyNotFound(name.clone()))?;
        match visited {
            VisitStates::UnVisisted => {}
            VisitStates::Visiting => {
                return Err(CycleDetected((name.to_owned(), module.name.clone())));
            }
            VisitStates::Visited => {
                continue;
            }
        }
        visit_map.insert(name.to_owned(), VisitStates::Visiting);
        let import_module = graph
            .modules
            .get(name)
            .ok_or(DependnecyNotFound(name.to_owned()))?;
        dfs_helper(graph, import_module, visit_map)?;
        visit_map.insert(name.clone(), VisitStates::Visited);
    }
    Ok(())
}
