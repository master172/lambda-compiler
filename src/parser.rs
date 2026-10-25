use crate::{
    lexer::definitions::Token::{
        self, Assignment, EOF, Lambda, LeftParen, RightParen, SemiColon, Var,
    },
    parser::{
        definitions::{
            AbstractionASTNode, ApplicationASTNode, Atom, AtomicASTNode, DefinitionASTNode,
            ExpressionASTNode,
            Expressions::{Abstraction, Application},
            IdentifierASTNode,
            ParserErrors::{
                self, ExpectedAssignmentToken, ExpectedDotToken, ExpectedIdentiferToken,
                ExpectedRightParenthesisToken, ExpectedSemiColon, InvalidAbstraction,
                InvalidApplication, InvalidAtomic, InvalidDefinition, InvalidExpression,
                InvalidStatmentInitalizer,
            },
            ProgramASTNode, StatementASTNode,
            Statemets::{Definition, StatementExpression},
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

//TODO add semi colon consumption here
fn parse_statement_node(state: &mut ParserState) -> Result<Option<StatementASTNode>, ParserErrors> {
    let token = state.peek();
    //eof reached stop parsing
    let token = match token {
        Some(val) => val,
        None => return Ok(None),
    };
    let statement = match token {
        EOF => {
            state.advance();
            return Ok(None);
        }
        Lambda { .. } => {
            let expression: ExpressionASTNode = parse_expression_node(state)?;
            let statement: StatementASTNode = StatementASTNode {
                statement: StatementExpression(expression),
            };
            statement
        }
        Var { .. } => match state.peek_n(1) {
            Some(Assignment { .. }) => {
                let definition: DefinitionASTNode = parse_defintion_node(state)?;
                let statement: StatementASTNode = StatementASTNode {
                    statement: Definition(definition),
                };
                statement
            }
            _ => {
                let expression: ExpressionASTNode = parse_expression_node(state)?;
                let statement: StatementASTNode = StatementASTNode {
                    statement: StatementExpression(expression),
                };
                statement
            }
        },
        LeftParen { .. } => {
            let expression: ExpressionASTNode = parse_expression_node(state)?;
            let statement: StatementASTNode = StatementASTNode {
                statement: StatementExpression(expression),
            };
            statement
        }
        _ => return Err(InvalidStatmentInitalizer),
    };

    //check for ending with semi colon
    let semi_colon = state.advance();
    match semi_colon {
        Some(SemiColon { .. }) => {}
        _ => return Err(ExpectedSemiColon),
    };
    Ok(Some(statement))
}

fn parse_defintion_node(state: &mut ParserState) -> Result<DefinitionASTNode, ParserErrors> {
    let identifier = state.advance();
    let identifier = match identifier {
        Some(Var {
            source: _source,
            attribute,
        }) => IdentifierASTNode {
            name: attribute.to_string(),
        },
        _ => return Err(InvalidDefinition),
    };
    let assignment = state.advance();
    match assignment {
        Some(Assignment { .. }) => {}
        _ => return Err(ExpectedAssignmentToken),
    };
    let expression: ExpressionASTNode = parse_expression_node(state)?;
    let defenition: DefinitionASTNode = DefinitionASTNode {
        identifier: identifier,
        expression,
    };
    Ok(defenition)
}

fn parse_expression_node(state: &mut ParserState) -> Result<ExpressionASTNode, ParserErrors> {
    let distinguishing_token = state.peek();
    let expression_ast_node = match distinguishing_token {
        Some(Lambda { .. }) => {
            let abstraction: AbstractionASTNode = parse_abstraction_node(state)?;
            let expression: ExpressionASTNode = ExpressionASTNode {
                expression: definitions::Expressions::Abstraction(abstraction),
            };
            expression
        }
        Some(Var { .. }) => {
            let application: ApplicationASTNode = parse_application_node(state)?;
            let expression: ExpressionASTNode = ExpressionASTNode {
                expression: definitions::Expressions::Application(application),
            };
            expression
        }
        _ => return Err(InvalidExpression),
    };
    Ok(expression_ast_node)
}

fn parse_abstraction_node(state: &mut ParserState) -> Result<AbstractionASTNode, ParserErrors> {
    let lambda_token = state.advance();
    match lambda_token {
        Some(Lambda { .. }) => {}
        _ => return Err(InvalidAbstraction),
    };
    let identifier = state.advance();
    let identifier = match identifier {
        Some(Var {
            source: _source,
            attribute,
        }) => IdentifierASTNode {
            name: attribute.to_string(),
        },
        _ => return Err(ExpectedIdentiferToken),
    };
    let dot = state.advance();
    match dot {
        Some(Token::Dot { .. }) => {}
        _ => return Err(ExpectedDotToken),
    };
    let expression: ExpressionASTNode = parse_expression_node(state)?;
    let expression = match expression.expression {
        Abstraction(abstraction) => Abstraction(abstraction),
        Application(application) => Application(application),
    };
    let abstraction: AbstractionASTNode = AbstractionASTNode {
        identifier,
        expression: Box::new(expression),
    };
    Ok(abstraction)
}

fn parse_application_node(state: &mut ParserState) -> Result<ApplicationASTNode, ParserErrors> {
    let left = parse_atomic_node(state)?;
    let mut application: ApplicationASTNode = ApplicationASTNode {
        left: Box::new(left),
        right: vec![],
    };
    while let Some(token) = state.peek() {
        match token {
            Var { .. } | LeftParen { .. } => {
                let right = parse_atomic_node(state)?;
                application.right.push(Box::new(right));
            }
            _ => break,
        };
    }

    Ok(application)
}

fn parse_atomic_node(state: &mut ParserState) -> Result<AtomicASTNode, ParserErrors> {
    let atomic = state.advance();
    let atomic = match atomic {
        Some(Var {
            source: _source,
            attribute,
        }) => {
            let identifier: IdentifierASTNode = IdentifierASTNode {
                name: attribute.to_string(),
            };
            let atomic: AtomicASTNode = AtomicASTNode {
                atom: Atom::Identifier(identifier),
            };
            atomic
        }
        Some(LeftParen { .. }) => {
            let expression: ExpressionASTNode = parse_expression_node(state)?;
            let atomic: AtomicASTNode = AtomicASTNode {
                atom: Atom::AtomicExpression(expression),
            };
            //consuming the right parenthesis
            let right_paren = state.advance();
            match right_paren {
                Some(RightParen { .. }) => {}
                _ => return Err(ExpectedRightParenthesisToken),
            };
            atomic
        }
        _ => return Err(InvalidAtomic),
    };

    Ok(atomic)
}
