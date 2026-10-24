use std::{fs::File, io::Error};

use crate::arguement_processor::args_parser::ArgsPacket;

pub fn get_source_file(packet: &ArgsPacket) -> Result<File, Error> {
    let file = File::open(packet.source.to_string())?;
    Ok(file)
}
