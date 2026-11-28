use std::{
    collections::HashMap,
    fmt,
    io::{self, BufRead, BufReader, Write},
};

use crate::{
    lexer::{
        self,
        definitions::{LexErrors, Token},
    },
    parser::{
        self,
        definitions::{
            ExpressionASTNode,
            Expressions::{Abstraction, Application},
            ParserErrors, ProgramASTNode,
            Statemets::{Definition, ImportStatement, StatementExpression},
        },
    },
    runtime::definitions::RuntimeErrors::{
        FailedAbstractionPrint, InputError, Io, Lex, Parse, Semantics, UnknownStream,
    },
    semantic_analyser::{
        canonizer::{Canonicalizer, definitions::CanoniclaizationContext},
        definitions::{SemanticErrors, SemanticStore},
    },
};

pub enum RuntimeErrors {
    FailedAbstractionPrint,
    UnknownStream(i64),
    Io(io::Error),
    Lex(LexErrors),
    Parse(ParserErrors),
    Semantics(SemanticErrors),
    InputError(String),
}

impl fmt::Display for RuntimeErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            FailedAbstractionPrint => write!(f, "failed to print phi abstraction"),
            UnknownStream(stream) => write!(f, "unknown stream: {stream}"),
            Io(err) => write!(f, "Io error {err}"),
            Lex(err) => write!(f, "Lexer Error {err}"),
            Parse(err) => write!(f, "Parser Error {err}"),
            Semantics(err) => write!(f, "Semantics Error {err}"),
            InputError(err) => write!(f, "failed converting input to ast with message: {err}"),
        }
    }
}
pub trait PhiValid {
    fn phi_string(&self) -> Result<String, RuntimeErrors>;
}

pub trait InputReducable {
    fn input_reduce<R: Runtime>(&self, runtime: &mut R)
    -> Result<ExpressionASTNode, RuntimeErrors>;
}

pub trait Runtime {
    fn phi<P: PhiValid>(&mut self, stream_id: i64, value: &P) -> Result<(), RuntimeErrors>;
    fn psi(&mut self, stream_id: i64) -> Result<ExpressionASTNode, RuntimeErrors>;
}

pub struct BasicRuntime {
    pub output_buffers: HashMap<i64, Box<dyn Write>>,
    pub input_buffers: HashMap<i64, Box<dyn BufRead>>,
    semantic_store: SemanticStore,
}

impl BasicRuntime {
    pub fn new(store: SemanticStore) -> BasicRuntime {
        let mut output_buffers: HashMap<i64, Box<dyn Write>> = HashMap::new();
        output_buffers.insert(1, Box::new(io::stdout()));
        output_buffers.insert(2, Box::new(io::stderr()));
        let mut input_buffers: HashMap<i64, Box<dyn BufRead>> = HashMap::new();
        input_buffers.insert(0, Box::new(BufReader::new(io::stdin())));
        BasicRuntime {
            output_buffers,
            input_buffers,
            semantic_store: store,
        }
    }
}

impl Runtime for BasicRuntime {
    fn phi<P: PhiValid>(&mut self, stream_id: i64, value: &P) -> Result<(), RuntimeErrors> {
        let output = value.phi_string()?;
        let stream = self
            .output_buffers
            .get_mut(&stream_id)
            .ok_or(RuntimeErrors::UnknownStream(stream_id))?;

        stream
            .write_all(output.as_bytes())
            .map_err(|err| RuntimeErrors::Io(err))?;

        Ok(())
    }

    fn psi(&mut self, stream_id: i64) -> Result<ExpressionASTNode, RuntimeErrors> {
        let mut input = String::new();

        let bytes_read = self
            .input_buffers
            .get_mut(&stream_id)
            .ok_or(RuntimeErrors::UnknownStream(stream_id))?
            .read_line(&mut input)
            .map_err(|err| Io(err))?;

        if bytes_read == 0 {
            return Err(InputError("unexpected end of input".into()));
        }

        let input = input.trim_end_matches(['\r', '\n']).to_string();

        // first basic parsing
        let tokens: Vec<Token> = lexer::lex_string(input).map_err(|err| Lex(err))?;
        let mut input_ast: ProgramASTNode = parser::parse(tokens).map_err(|err| Parse(err))?;

        // now verification of the ast
        if input_ast.statements.is_empty() {
            return Err(InputError(String::from("input contains no statements")));
        }
        if input_ast.statements.len() > 1 {
            return Err(InputError(String::from(
                "input cannot contain more than one statement",
            )));
        }

        // now semantic specific work (Canonicalization of names)
        let context: CanoniclaizationContext = CanoniclaizationContext::new(&self.semantic_store);
        let mut canonizer = Canonicalizer::new(context);
        canonizer
            .canonicalize(&mut self.semantic_store, &mut input_ast)
            .map_err(|err| Semantics(err))?;

        // now we handle verifying the type of the ast node
        let expression_astnode = match &input_ast.statements[0].statement {
            Definition(_) => {
                return Err(InputError(String::from("defintions not allowed in input")));
            }
            ImportStatement(_) => {
                return Err(InputError(String::from("imports not allowed in input")));
            }
            StatementExpression(expression) => expression,
        };
        match &expression_astnode.expression {
            Abstraction(_) => {}
            Application(_) => {}
            _ => {
                return Err(InputError(String::from(
                    "only regualr lambda expressions are allowed in input",
                )));
            }
        };
        Ok(expression_astnode.clone())
    }
}
