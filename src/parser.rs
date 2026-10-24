use crate::{
    lexer::definitions::Token::{self, EOF, Lambda, LeftParen, Var},
    parser::{
        definitions::{
            AbstractionASTNode, DefinitionASTNode, ExpressionASTNode,
            Expressions::Abstraction,
            ParserErrors::{self, InvalidStatmentInitalizer},
            ProgramASTNode, StatementASTNode,
            Statemets::StatementExpression,
        },
        state::ParserState,
    },
};

mod definitions;
mod state;

pub fn parse(tokens: Vec<Token>) -> Result<ProgramASTNode, ParserErrors> {
    let mut state: ParserState = ParserState::new(tokens);
    let node = match parse_program_node(&mut state)? {
        Some(val) => val,
        None => return Err(ParserErrors::EOFReachedWithoutParsing),
    };
    return Ok(node);
}

fn parse_program_node(state: &mut ParserState) -> Result<Option<ProgramASTNode>, ParserErrors> {
    let mut program = ProgramASTNode { statements: vec![] };
    while let Some(statment) = parse_statement_node(state)? {
        program.statements.push(statment);
    }

    return Ok(Some(program));
}

fn parse_statement_node(state: &mut ParserState) -> Result<Option<StatementASTNode>, ParserErrors> {
    let token = state.peek();
    //eof reached stop parsing
    let token = match token {
        Some(val) => val,
        None => return Ok(None),
    };
    match token {
        EOF => return Ok(None),
        Lambda { .. } => {
            let abstraction: AbstractionASTNode = parse_abstraction_node(state)?;
            let expression: ExpressionASTNode = ExpressionASTNode {
                expression: Abstraction(abstraction),
            };
            let statement: StatementASTNode = StatementASTNode {
                statement: StatementExpression(expression),
            };
            return Ok(Some(statement));
        }
        Var { source, attribute } => {
            todo!()
        }
        LeftParen { source } => {
            todo!()
        }
        _ => return Err(InvalidStatmentInitalizer),
    };
}

fn parse_defintion_node(state: &mut ParserState) -> Result<DefinitionASTNode, ParserErrors> {
    todo!()
}

fn parse_expression_node(state: &mut ParserState) -> Result<ExpressionASTNode, ParserErrors> {
    todo!()
}

fn parse_abstraction_node(state: &mut ParserState) -> Result<AbstractionASTNode, ParserErrors> {
    todo!()
}
