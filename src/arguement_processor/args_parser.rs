use std::{env, fmt::Display, path::PathBuf};

use crate::{
    args_parser::ArgsError::{InvalidArguements, NoDirectory, NoSourceProvided},
    arguement_processor::args_parser::ArgsError::InvalidIncludes,
};

pub struct ArgsPacket {
    pub source: PathBuf,
    pub modules: Vec<PathBuf>,
    pub allow_free_variables: bool,
    pub debug: bool,
    pub defintion_print: bool,
}

pub enum ArgsError {
    NoDirectory,
    InvalidIncludes,
    InvalidArguements,
    NoSourceProvided,
}

impl Display for ArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NoDirectory => write!(f, "No Directory passed to the compiler"),
            InvalidIncludes => write!(f, "Invalid include file arguements passed to the compiler"),
            InvalidArguements => write!(f, "Invalid Arguements passed to the compiler"),
            NoSourceProvided => write!(f, "No Source file provided to the compiler"),
        }
    }
}

pub fn create_args_packet() -> Result<ArgsPacket, ArgsError> {
    let mut args = env::args().peekable();

    //get the current dir if somehow it is empty return an error
    let path = env::current_dir().map_err(|_| NoDirectory)?;

    let mut source_packet: ArgsPacket = ArgsPacket {
        source: PathBuf::new(),
        modules: Vec::new(),
        allow_free_variables: false,
        debug: false,
        defintion_print: false,
    };

    while let Some(value) = args.next() {
        // go over each space sperated value in the env args and modify source packet values from it
        // since we use iterators it is easy to use parameter specifiers and actual values
        if value == "-s" {
            // immediately get the next arguement as the source file path from the current directory
            let source_file: String = match args.next() {
                Some(val) => val,
                None => return Err(NoSourceProvided),
            };

            source_packet.source = path.join(source_file);
        }
        if value == "-f" {
            source_packet.allow_free_variables = true
        }
        if value == "-d" {
            source_packet.debug = true
        }
        if value == "-pd" {
            source_packet.defintion_print = true
        }
        if value == "-m" {
            while let Some(arg) = args.peek() {
                if arg.starts_with('-') {
                    break;
                }
                let source_file = match args.next() {
                    Some(val) => val,
                    None => return Err(InvalidIncludes),
                };
                source_packet.modules.push(path.join(source_file));
            }
        }
    }

    //validate if the source is proper if not immediately return an error
    if source_packet.source.is_empty() {
        return Err(InvalidArguements);
    }

    return Ok(source_packet);
}
