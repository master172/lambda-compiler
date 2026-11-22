use std::{collections::HashMap, env, fmt, path::PathBuf};

use crate::{
    arguement_processor::args_parser::{self, ArgsError},
    compiler_definitions::FormatErrors::{NoDirectory, PathConversionError},
    lexer::{self, definitions::LexErrors},
    parser::{
        self,
        definitions::{ParserErrors, ProgramASTNode},
    },
};

pub enum FormatErrors {
    PathConversionError(String),
    NoDirectory,
}

impl fmt::Display for FormatErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NoDirectory => write!(f, "no directory for curren module"),
            PathConversionError(path) => write!(f, "failed to parse path {path} to string"),
        }
    }
}

pub enum CompilerErrors {
    ArgumentErrors(args_parser::ArgsError),
    ParserErrors(parser::definitions::ParserErrors),
    LexerErrors(lexer::definitions::LexErrors),
    FormatErrors(FormatErrors),
}

impl fmt::Display for CompilerErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArgumentErrors(err) => write!(f, "{err}"),
            Self::ParserErrors(err) => write!(f, "{err}"),
            Self::LexerErrors(err) => write!(f, "{err}"),
            Self::FormatErrors(err) => write!(f, "{err}"),
        }
    }
}

impl From<ArgsError> for CompilerErrors {
    fn from(value: ArgsError) -> Self {
        Self::ArgumentErrors(value)
    }
}

impl From<ParserErrors> for CompilerErrors {
    fn from(value: ParserErrors) -> Self {
        Self::ParserErrors(value)
    }
}

impl From<LexErrors> for CompilerErrors {
    fn from(value: LexErrors) -> Self {
        Self::LexerErrors(value)
    }
}

impl From<FormatErrors> for CompilerErrors {
    fn from(value: FormatErrors) -> Self {
        Self::FormatErrors(value)
    }
}

#[derive(Debug)]
pub struct ModuleStore {
    pub modules: HashMap<String, ProgramASTNode>,
}

impl ModuleStore {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }
    pub fn format_to_namespaced(self) -> Result<ModuleStore, FormatErrors> {
        let mut module_store: ModuleStore = ModuleStore {
            modules: HashMap::new(),
        };
        for (name, module) in self.modules {
            let formatted_name = normalize_identifier(&name)?;
            module_store.modules.insert(formatted_name, module);
        }
        Ok(module_store)
    }
}

pub fn normalize_identifier(identifier: &str) -> Result<String, FormatErrors> {
    let root = env::current_dir().map_err(|_| FormatErrors::NoDirectory)?;
    let path = PathBuf::from(identifier);
    let relative = path
        .strip_prefix(&root)
        .map_err(|_| FormatErrors::PathConversionError(identifier.to_string()))?;
    let relative = relative.with_extension("");

    Ok(relative
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("->"))
}
