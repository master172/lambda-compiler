use crate::lexer::definitions::Token;

pub struct ParserState {
    tokens: Vec<Token>,
    current: usize,
}

impl ParserState {
    pub fn new(tokens: Vec<Token>) -> Self {
        return Self { tokens, current: 0 };
    }

    pub fn peek(&self) -> Option<&Token> {
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
