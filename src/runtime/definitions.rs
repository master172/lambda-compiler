use std::{collections::HashMap, fmt};

use crate::runtime::definitions::RuntimeErrors::FailedAbstractionPrint;

#[derive(Debug)]
pub enum RuntimeErrors {
    FailedAbstractionPrint,
}

impl fmt::Display for RuntimeErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            FailedAbstractionPrint => write!(f, "failed to print phi abstraction"),
        }
    }
}
pub trait PhiValid {
    fn phi_string(&self) -> Result<String, RuntimeErrors>;
}

pub trait Runtime {
    fn phi<P: PhiValid>(&mut self, stream_id: i64, value: &P) -> Result<(), RuntimeErrors>;
}

pub struct BasicRuntime {
    pub output_buffers: HashMap<i64, String>,
}

impl BasicRuntime {
    pub fn new() -> BasicRuntime {
        let mut output_buffers: HashMap<i64, String> = HashMap::new();
        output_buffers.insert(1, String::new());
        output_buffers.insert(2, String::new());
        BasicRuntime { output_buffers }
    }
}

impl Runtime for BasicRuntime {
    fn phi<P: PhiValid>(&mut self, stream_id: i64, value: &P) -> Result<(), RuntimeErrors> {
        let output = value.phi_string()?;
        self.output_buffers
            .entry(stream_id)
            .or_default()
            .push_str(&output);
        Ok(())
    }
}
