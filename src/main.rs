use std::fs::File;

use crate::{
    arguement_processor::{
        args_parser::{self, ArgsPacket},
        packet_processor,
    },
    lexer::definitions::Token,
};

mod arguement_processor;
mod lexer;

fn main() {
    // gets the arguement packet from the args parser
    let args_packet: Result<ArgsPacket, args_parser::ArgsError> = args_parser::create_args_packet();
    let args_packet: ArgsPacket = match args_packet {
        Ok(val) => val,
        Err(err) => panic!("compiler exited with error: {}", err),
    };

    // opens the relavent source file
    let file: File = match packet_processor::get_source_file(&args_packet) {
        Ok(val) => val,
        Err(err) => panic!("failed to open file: {}", err),
    };

    //tries to get a stream of tokens from the lexer
    let tokens: Vec<Token> = match lexer::lex(file) {
        Ok(val) => val,
        Err(e) => panic!("lexer failed with error: {e}"),
    };
    println!("{:#?}", tokens)
}
