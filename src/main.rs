use std::{fs::File, path::PathBuf};

use crate::{
    arguement_processor::{
        args_parser::{self, ArgsPacket},
        packet_processor,
    },
    compiler_definitions::{CompilerErrors, ModuleStore},
    dependency_graph::helpers::depth_first_search,
    lexer::definitions::Token,
    parser::definitions::ProgramASTNode,
    pretty_printer::definitions::PrettyPrinter,
    semantic_analyser::{create_semantic_table, definitions::Analyser},
    tree_evaluator::evaluate,
    visitor::definitons::Visitor,
};

pub mod arguement_processor;
mod compiler_definitions;
mod dependency_graph;
pub mod lexer;
pub mod parser;
mod pretty_printer;
mod semantic_analyser;
mod tree_evaluator;
mod visitor;

fn main() {
    // gets the arguement packet from the args parser
    let args_packet: Result<ArgsPacket, args_parser::ArgsError> = args_parser::create_args_packet();
    let args_packet: ArgsPacket = match args_packet {
        Ok(val) => val,
        Err(err) => panic!("compiler exited with error: {}", err),
    };
    let mut program_ast = match parse_file(&args_packet.source) {
        Ok(val) => val,
        Err(e) => panic!("compiler failed with error {e}"),
    };
    let program_name: String = match args_packet.source.into_string() {
        Ok(val) => val,
        Err(_) => panic!("failed to find main module name"),
    };

    //create the AST for all the include files
    let mut program_includes: ModuleStore = ModuleStore::new();
    for argument in args_packet.modules.iter() {
        let module_ast = match parse_file(&argument) {
            Ok(val) => val,
            Err(e) => panic!("compiler failed with error {e}"),
        };
        program_includes.modules.insert(
            argument
                .to_owned()
                .into_string()
                .expect("failed to convert path buf to string")
                .to_string(),
            module_ast,
        );
    }
    //println!("module store: {:#?}", program_includes);
    let dependency_graph = match dependency_graph::create_dependency_graph(
        &program_ast,
        &program_name,
        &program_includes,
    ) {
        Ok(val) => val,
        Err(e) => panic!("failed to create dependency graph with error {e}"),
    };
    //println!("Dependency graph {:#?}", dependency_graph);
    let topological_dependnecy_array = match depth_first_search(&dependency_graph) {
        Ok(val) => val,
        Err(err) => panic!("Dependency error detected {err}"),
    };
    // run the semantic checker on the ast
    let semantic_table = match create_semantic_table(&program_ast) {
        Ok(result) => result,
        Err(err) => panic!("semantic table building failed with error: {}", err),
    };
    let mut semantic_visitor = Analyser::new(&semantic_table, args_packet.allow_free_variables);
    match semantic_visitor.visit_program(&program_ast) {
        Ok(_) => {}
        Err(err) => panic!("semantic table building failed with error: {}", err),
    };
    // semantic analysyis completed we can move to interpetation or code gen from here

    // we just evaluat the AST directly here
    evaluate(&mut program_ast, &semantic_table, args_packet.debug);

    // we can print it using a pretty printer
    let mut pretty_printer: PrettyPrinter = PrettyPrinter {
        print_defintion: args_packet.defintion_print,
    };
    match pretty_printer.visit_program(&program_ast) {
        Ok(str) => println!("{}", str),
        _ => eprintln!("pretty printer encountered an error"),
    };
    //println!("{:#?}", program_ast)
}

fn parse_file(file: &PathBuf) -> Result<ProgramASTNode, CompilerErrors> {
    let file: File = match packet_processor::get_file(file) {
        Ok(val) => val,
        Err(err) => panic!("failed to open file: {}", err),
    };

    //tries to get a stream of tokens from the lexer
    let tokens: Vec<Token> = match lexer::lex(file) {
        Ok(val) => val,
        Err(e) => panic!("lexer failed with error: {e}"),
    };

    // try to parse the stream of tokens into an AST
    let module_ast = match parser::parse(tokens) {
        Ok(ast) => ast,
        Err(err) => panic!("parser failed with error: {}", err),
    };

    Ok(module_ast)
}
