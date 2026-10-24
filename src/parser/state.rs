use crate::lexer::definitions::Token;

pub struct ParserState {
    tokens: Vec<Token>,
    current: usize,
    start: usize,
    end: usize,
}

impl ParserState {
    pub fn new(tokens: Vec<Token>) -> Self {
        let token_len: usize = tokens.len();
        return Self {
            tokens,
            current: 0,
            start: 0,
            end: token_len,
        };
    }

    pub fn peek(&mut self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    pub fn peek_n(&mut self, num: usize) -> Option<&Token> {
        self.tokens.get(self.current + num)
    }

    pub fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.current)?;
        self.current += 1;
        Some(token)
    }
}
