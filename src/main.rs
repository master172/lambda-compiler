use std::fs::File;

use crate::{
    arguement_processor::{
        args_parser::{self, ArgsPacket},
        packet_processor,
    },
    lexer::definitions::Token,
    semantic_analyser::{create_semantic_table, definitions::Analyser},
    visitor::definitons::Visitor,
};

mod arguement_processor;
mod lexer;
mod parser;
mod semantic_analyser;
mod visitor;

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

    // try to parse the stream of tokens into an AST
    let program_ast = match parser::parse(tokens) {
        Ok(ast) => ast,
        Err(err) => panic!("parser failed with error: {}", err),
    };

    // run the semantic checker on the ast
    let semantic_table = match create_semantic_table(&program_ast) {
        Ok(result) => result,
        Err(err) => panic!("semantic table building failed with error: {}", err),
    };
    let mut semantic_visitor = Analyser::new(&semantic_table);
    match semantic_visitor.visit_program(&program_ast) {
        Ok(()) => {}
        Err(err) => panic!("semantic table building failed with error: {}", err),
    };

    // semantic analysyis completed we can move to interpetation or code gen from here
    println!("{:#?}", program_ast)
}
