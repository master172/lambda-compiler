use std::collections::HashMap;

use crate::{
    parser::definitions::{ProgramASTNode, Statemets::Definition},
    semantic_analyser::definitions::{
        SemanticErrors::{self, DuplicateDefinition},
        SemanticTable,
    },
};

pub mod definitions;

//no need for a tree evaluator here since definitons are statements
pub fn create_semantic_table<'a>(
    program: &'a ProgramASTNode,
) -> Result<SemanticTable, SemanticErrors> {
    let mut semantic_table: SemanticTable = SemanticTable {
        definitions: HashMap::new(),
    };
    for statement in program.statements.iter() {
        match &statement.statement {
            Definition(definition) => {
                if semantic_table
                    .definitions
                    .contains_key(&definition.identifier.name)
                {
                    return Err(DuplicateDefinition(definition.identifier.name.to_owned()));
                }
                semantic_table
                    .definitions
                    .insert(definition.identifier.name.clone(), definition.clone());
            }
            _ => {}
        }
    }

    return Ok(semantic_table);
}
