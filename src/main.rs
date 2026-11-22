use std::{fs::File, path::PathBuf};

use crate::{
    arguement_processor::{
        args_parser::{self, ArgsPacket},
        packet_processor,
    },
    compiler_definitions::{CompilerErrors, ModuleStore, normalize_identifier},
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
    //create the dependnecy graph form the module store, and program ast
    let dependency_graph = match dependency_graph::create_dependency_graph(
        &program_ast,
        &program_name,
        &program_includes,
    ) {
        Ok(val) => val,
        Err(e) => panic!("failed to create dependency graph with error {e}"),
    };
    // search for cyclic imports and create a topological sorted array for dependencies
    let mut topological_dependnecy_array = match depth_first_search(&dependency_graph) {
        Ok(val) => val,
        Err(err) => panic!("Dependency error detected {err}"),
    };
    // format the module store to use namespaced keys instead of directories
    program_includes = match program_includes.format_to_namespaced() {
        Err(e) => panic!("module store formatting error {e}"),
        Ok(val) => val,
    };
    // remove the source file from the topological array since a global table is created for it seperately
    let formatted_program_name: String = normalize_identifier(&program_name)
        .unwrap_or_else(|err| panic!("could not format program name {err}"));
    topological_dependnecy_array.retain(|x| x != &formatted_program_name);
    // create the proper semantic store
    let semantic_store = match create_semantic_table(
        &mut program_includes,
        &topological_dependnecy_array,
        &program_ast,
    ) {
        Ok(result) => result,
        Err(err) => panic!("semantic store building failed with error: {}", err),
    };
    let mut semantic_visitor = Analyser::new(&semantic_store, args_packet.allow_free_variables);
    match semantic_visitor.analyse(&program_ast) {
        Ok(_) => {}
        Err(err) => panic!("semantic store analysis failed with error: {}", err),
    };
    // semantic analysyis completed we can move to interpetation or code gen from here
    // first flatten the store
    let semantic_table = semantic_store.flatten();
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
