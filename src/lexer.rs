use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::lexer::{
    definitions::{
        LexErrors::{self, FailedToProcessLine, InvalidIdentifier},
        SourceLocation, Token,
    },
    helpers::is_valid_identifier_name,
};

pub mod definitions;
pub mod helpers;

struct LexState {
    start: usize,
    current: usize,
    end: usize,
}

impl LexState {
    fn new(line: &str) -> Self {
        LexState {
            start: 0,
            current: 0,
            end: line.len(),
        }
    }
}
pub fn lex(file: File) -> Result<Vec<Token>, LexErrors> {
    let mut returning_token: Vec<Token> = Vec::new();

    let reader: BufReader<File> = BufReader::new(file);
    for (line_num, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(val) => val,
            Err(_) => return Err(FailedToProcessLine),
        };
        match lex_line(&line, line_num) {
            Ok(mut val) => returning_token.append(&mut val),
            Err(e) => return Err(e),
        };
    }

    Ok(returning_token)
}

pub fn lex_line(line: &str, line_num: usize) -> Result<Vec<Token>, LexErrors> {
    let chars: Vec<char> = line.chars().collect();
    let mut returning_token: Vec<Token> = vec![];
    let mut lex_state: LexState = LexState::new(&line);
    while lex_state.current < lex_state.end {
        match chars[lex_state.current] {
            '.' => {
                lex_state.start = lex_state.current;
                lex_state.current += 1;
                returning_token.push(Token::Dot {
                    source: SourceLocation::new(
                        line_num,
                        lex_state.start,
                        lex_state.current - lex_state.start,
                    ),
                });
            }
            '(' => {
                lex_state.start = lex_state.current;
                lex_state.current += 1;
                returning_token.push(Token::LeftParen {
                    source: SourceLocation::new(
                        line_num,
                        lex_state.start,
                        lex_state.current - lex_state.start,
                    ),
                });
            }
            ')' => {
                lex_state.start = lex_state.current;
                lex_state.current += 1;
                returning_token.push(Token::RightParen {
                    source: SourceLocation::new(
                        line_num,
                        lex_state.start,
                        lex_state.current - lex_state.start,
                    ),
                });
            }

            c if c.is_whitespace() || c.is_control() => {
                lex_state.start = lex_state.current;
                lex_state.current += 1
            }
            _ => match lex_identifier(&mut lex_state, &chars, line_num) {
                Ok(token) => {
                    returning_token.push(token);
                }
                Err(err) => return Err(err),
            },
        }
    }
    returning_token.push(Token::EOF {
        source: SourceLocation::new(
            line_num,
            lex_state.start,
            lex_state.current - lex_state.start,
        ),
    });
    Ok(returning_token)
}

fn lex_identifier(
    state: &mut LexState,
    char_vec: &Vec<char>,
    line: usize,
) -> Result<Token, LexErrors> {
    state.start = state.current;
    while state.current < state.end {
        let c = char_vec[state.current];
        if c.is_whitespace() || c == '.' {
            break;
        }
        state.current += 1;
    }
    let key_slice: &[char] = &char_vec[state.start..state.current];
    let key_str: String = key_slice.iter().collect();
    if !is_valid_identifier_name(&key_str) {
        return Err(InvalidIdentifier);
    }
    match match_key_word_to_token(&key_str, line, state.current, state.start) {
        Some(val) => return Ok(val),
        None => {}
    }
    return Ok(Token::Var {
        source: SourceLocation::new(line, state.start, state.current - state.start),
        attribute: key_str,
    });
}

pub fn match_key_word_to_token(
    key_word: &str,
    line: usize,
    column: usize,
    start: usize,
) -> Option<Token> {
    match key_word {
        "lambda" => Some(Token::Lambda {
            source: SourceLocation::new(line, start, column - start),
        }),
        _ => None,
    }
}
