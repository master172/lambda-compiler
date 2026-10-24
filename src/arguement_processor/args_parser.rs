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
    let path = env::current_dir().map_err(|_| NoDirectory)?;
    let path_string = match path.into_string() {
        Ok(val) => val,
        Err(_) => return Err(InvalidDir),
    };
    let mut source_packet: Option<ArgsPacket> = None;
    while let Some(value) = args.next() {
        if value == "-s" {
            let source_file: String = match args.next() {
                Some(val) => val,
                None => return Err(NoSourceProvided),
            };

            source_packet = Some(ArgsPacket {
                source: format!("{path_string}\\{source_file}"),
            });
        }
    }
    if source_packet.is_some() {
        return Ok(source_packet.unwrap());
    }
    Err(InvalidArguements)
}
