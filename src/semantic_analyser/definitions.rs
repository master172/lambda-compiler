use std::collections::HashMap;

use crate::parser::definitions::DefinitionASTNode;

pub enum SemanticErrors<'a> {
    UndeclaredIdentifier(&'a str),
    DuplicateDefinition(&'a str),
}

pub struct SemanticTable<'a> {
    pub definitions: HashMap<&'a str, &'a DefinitionASTNode>,
}
