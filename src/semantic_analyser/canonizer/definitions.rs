use std::collections::{HashMap, HashSet};

use crate::semantic_analyser::definitions::SemanticStore;

pub struct CanoniclaizationContext {
    global_defintions: HashSet<String>,
    module_definitions: HashMap<String, HashSet<String>>,
    global_imports: HashSet<String>,
    module_imports: HashMap<String, HashSet<String>>,
}

impl CanoniclaizationContext {
    pub fn new(store: &SemanticStore) -> Self {
        Self {
            global_defintions: store.global.definitions.keys().cloned().collect(),
            module_definitions: store
                .modules
                .iter()
                .map(|(module, table)| {
                    (module.clone(), table.definitions.keys().cloned().collect())
                })
                .collect(),
            global_imports: store.global_imports.clone(),
            module_imports: store.module_imports.clone(),
        }
    }

    pub fn resolve_identifier(&self, name: &str, current_module: Option<&str>) -> Option<String> {
        if let Some((module, definition)) = name.rsplit_once("->") {
            let visible = match current_module {
                Some(current) if current == module => true,
                Some(current) => self
                    .module_imports
                    .get(current)
                    .is_some_and(|import| import.contains(module)),
                None => self.global_imports.contains(module),
            };

            if visible
                && self
                    .module_definitions
                    .get(module)
                    .is_some_and(|definitions| definitions.contains(definition))
            {
                return Some(name.to_owned());
            }
            return None;
        }

        // get the scoped table if the name was not qualified
        let local_definitons = match current_module {
            Some(module) => self.module_definitions.get(module)?,
            None => &self.global_defintions,
        };

        if local_definitons.contains(name) {
            return Some(match current_module {
                Some(module) => format!("{module}->{name}"),
                None => name.to_owned(),
            });
        }

        // if not in local scope as well see the imports
        let imports = match current_module {
            Some(module) => self.module_imports.get(module),
            None => Some(&self.global_imports),
        };

        let mut resolved: Option<String> = None;
        if let Some(imports) = imports {
            for imported_module in imports {
                let definitions = self.module_definitions.get(imported_module)?;

                if definitions.contains(name) {
                    if resolved.is_some() {
                        return None;
                    }
                    resolved = Some(format!("{imported_module}->{name}"));
                }
            }
        }
        resolved
    }
}
