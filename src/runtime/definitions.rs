use std::{
    collections::HashMap,
    fmt,
    io::{self, Write},
};

use crate::{
    parser::definitions::ExpressionASTNode,
    runtime::definitions::RuntimeErrors::{FailedAbstractionPrint, Io, UnknownStream},
    semantic_analyser::definitions::SemanticTable,
};

#[derive(Debug)]
pub enum RuntimeErrors {
    FailedAbstractionPrint,
    UnknownStream(i64),
    Io(io::Error),
}

impl fmt::Display for RuntimeErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            FailedAbstractionPrint => write!(f, "failed to print phi abstraction"),
            UnknownStream(stream) => write!(f, "unknown stream: {stream}"),
            Io(err) => write!(f, "{err}"),
        }
    }
}
pub trait PhiValid {
    fn phi_string(&self) -> Result<String, RuntimeErrors>;
}

pub trait InputReducable {
    fn input_reduce<R: Runtime>(
        &self,
        definitions: &SemanticTable,
        runtime: &mut R,
    ) -> Result<ExpressionASTNode, RuntimeErrors>;
}

pub trait Runtime {
    fn phi<P: PhiValid>(&mut self, stream_id: i64, value: &P) -> Result<(), RuntimeErrors>;
    fn psi(
        &self,
        stream_id: i64,
        definitions: &SemanticTable,
    ) -> Result<ExpressionASTNode, RuntimeErrors>;
}

pub struct BasicRuntime {
    pub output_buffers: HashMap<i64, Box<dyn Write>>,
}

impl BasicRuntime {
    pub fn new() -> BasicRuntime {
        let mut output_buffers: HashMap<i64, Box<dyn Write>> = HashMap::new();
        output_buffers.insert(1, Box::new(io::stdout()));
        output_buffers.insert(2, Box::new(io::stderr()));
        BasicRuntime { output_buffers }
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

    fn psi(
        &self,
        stream_id: i64,
        definitions: &SemanticTable,
    ) -> Result<ExpressionASTNode, RuntimeErrors> {
        todo!()
    }
}
