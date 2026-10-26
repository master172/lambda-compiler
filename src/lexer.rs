use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::lexer::{
    definitions::{
        LexErrors::{self, FailedToProcessLine, InvalidIdentifier},
        SourceLocation, Token,
    },
    helpers::{is_identifier_terminator, is_valid_identifier_name},
};

pub mod definitions;
pub mod helpers;

/// internal struct to keep track of state
/// it is kept private since no other part of the program should interfere with the lexer
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
            end: line.chars().count(),
        }
    }
}

/// the lex function iterates over each line and gets the appropriate token vector for the line
/// returns any errors if the lex_line function returns a error
/// and finally adds the EOF token at the end
pub fn lex(file: File) -> Result<Vec<Token>, LexErrors> {
    let mut returning_token: Vec<Token> = Vec::new();

    let reader: BufReader<File> = BufReader::new(file);
    let mut lines_processed: usize = 0;
    for (line_num, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(val) => val,
            Err(_) => return Err(FailedToProcessLine),
        };
        match lex_line(&line, line_num) {
            Ok(mut val) => returning_token.append(&mut val),
            Err(e) => return Err(e),
        };
        lines_processed = line_num;
    }
    let last_column = match returning_token.last() {
        Some(val) => {
            let source = val.get_source();
            source.get_columm() + source.get_size()
        }
        None => 0,
    };
    returning_token.push(Token::EOF {
        source: SourceLocation::new(lines_processed, last_column, 0),
    });

    Ok(returning_token)
}

/// lex_line function converts a string slice to a vector of characters and then uses the LexState struct to keep track
/// of the current index that in the line that the lexer is processing
pub fn lex_line(line: &str, line_num: usize) -> Result<Vec<Token>, LexErrors> {
    let chars: Vec<char> = line.chars().collect();
    let mut returning_token: Vec<Token> = vec![];
    let mut lex_state: LexState = LexState::new(&line);
    while lex_state.current < lex_state.end {
        // single letter characters can be added here
        // so far there had been no need to validate multi character tokens such as != or <=
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
            '=' => {
                lex_state.start = lex_state.current;
                lex_state.current += 1;
                returning_token.push(Token::Assignment {
                    source: SourceLocation::new(
                        line_num,
                        lex_state.start,
                        lex_state.current - lex_state.start,
                    ),
                });
            }
            ';' => {
                lex_state.start = lex_state.current;
                lex_state.current += 1;
                returning_token.push(Token::SemiColon {
                    source: SourceLocation::new(
                        line_num,
                        lex_state.start,
                        lex_state.current - lex_state.start,
                    ),
                });
            }
            'λ' => {
                lex_state.start = lex_state.current;
                lex_state.current += 1;
                returning_token.push(Token::Lambda {
                    source: SourceLocation::new(
                        line_num,
                        lex_state.start,
                        lex_state.current - lex_state.start,
                    ),
                });
            }

            //this skips over and whitespace or control characrers such as space, tab or new line
            c if c.is_whitespace() || c.is_control() => {
                lex_state.start = lex_state.current;
                lex_state.current += 1
            }
            //all other characters are assumed to be the beginning of an identifier
            _ => match lex_identifier(&mut lex_state, &chars, line_num) {
                Ok(token) => {
                    returning_token.push(token);
                }
                Err(err) => return Err(err),
            },
        }
    }

    Ok(returning_token)
}

fn lex_identifier(
    state: &mut LexState,
    char_vec: &Vec<char>,
    line: usize,
) -> Result<Token, LexErrors> {
    state.start = state.current;

    //stop reading characters if the end of line is reached
    while state.current < state.end {
        let c = char_vec[state.current];
        //or stop reading if a syntactically valid character is found
        if is_identifier_terminator(c) {
            break;
        }
        state.current += 1;
    }

    //form a string from the start and current pointers
    let key_slice: &[char] = &char_vec[state.start..state.current];
    let key_str: String = key_slice.iter().collect();

    //assume it is a valid identifer name first then validate it
    if !is_valid_identifier_name(&key_str) {
        return Err(InvalidIdentifier);
    }

    //check if it is a keyword if yes return a key word token
    match match_key_word_to_token(&key_str, line, state.current, state.start) {
        Some(val) => return Ok(val),
        None => {}
    }

    //return an identifier token otherwise
    return Ok(Token::Var {
        source: SourceLocation::new(line, state.start, state.current - state.start),
        attribute: key_str,
    });
}

/// this function is repsonsible for mapping string tokens to keywords
pub fn match_key_word_to_token(
    key_word: &str,
    line: usize,
    column: usize,
    start: usize,
) -> Option<Token> {
    //keywords need to be added here
    match key_word {
        "lambda" => Some(Token::Lambda {
            source: SourceLocation::new(line, start, column - start),
        }),
        _ => None,
    }
}
