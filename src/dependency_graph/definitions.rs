use std::{
    collections::{HashMap, HashSet},
    fmt::{self},
};

use crate::dependency_graph::definitions::DependencyErrors::{
    ModuleNotProvided, NoDirectory, PathConversionError,
};

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub imports: HashSet<String>,
}

#[derive(Debug)]
pub struct DependencyGraph {
    pub modules: HashMap<String, Module>,
}

pub enum DependencyErrors {
    ModuleNotProvided(String),
    NoDirectory,
    PathConversionError(String),
}

impl fmt::Display for DependencyErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModuleNotProvided(module) => write!(f, "module not provided {}", module),
            NoDirectory => write!(f, "no directory for curren module"),
            PathConversionError(path) => write!(f, "failed to parse path {path} to string"),
        }
    }
}
