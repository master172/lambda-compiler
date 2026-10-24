use std::{env, fmt::Display};

use crate::args_parser::ArgsError::{InvalidArguements, InvalidDir, NoDirectory, NoSourceProvided};

pub struct ArgsPacket {
    pub source: String,
}

pub enum ArgsError {
    NoDirectory,
    InvalidDir,
    InvalidArguements,
    NoSourceProvided,
}

impl Display for ArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NoDirectory => write!(f, "No Directory passed to the compiler"),
            InvalidDir => write!(f, "Invalid Directory passed to the compiler"),
            InvalidArguements => write!(f, "Invalid Arguements passed to the compiler"),
            NoSourceProvided => write!(f, "No Source file provided to the compiler"),
        }
    }
}

pub fn create_args_packet() -> Result<ArgsPacket, ArgsError> {
    let mut args = env::args();

    //get the current dir if somehow it is empty return an error
    let path = env::current_dir().map_err(|_| NoDirectory)?;
    let path_string = match path.into_string() {
        Ok(val) => val,
        Err(_) => return Err(InvalidDir),
    };

    let mut source_packet: ArgsPacket = ArgsPacket {
        source: String::new(),
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

            source_packet.source = format!("{path_string}\\{source_file}");
        }
    }

    //validate if the source is proper if not immediately return an error
    if source_packet.source.is_empty() {
        return Err(InvalidArguements);
    }

    return Ok(source_packet);
}
