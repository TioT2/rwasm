mod decode_wasm;
mod opcode;

use std::collections::HashMap;

use crate::{types, Function};

pub enum Source<'t> {
    WASM(&'t [u8]),
    WAT(&'t str),
}

pub struct Module {
    pub(crate) types: Vec<types::FunctionType>,
    pub(crate) imports: HashMap<String, HashMap<String, types::ImportDescriptor>>,
    pub(crate) exports: HashMap<String, types::ExportDescriptor>,
    pub(crate) functions: Vec<Function>,
    pub(crate) start: Option<u32>,
}

impl Module {
    pub fn new(source: Source) -> Result<Self, ModuleCreateError> {
        match source {
            Source::WASM(bits) => Self::from_wasm(bits),
            Source::WAT(_) => Err(ModuleCreateError::Unknown),
        }
    }

    pub fn get_function_types(&self) -> &[types::FunctionType] {
        &self.types
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum ModuleCreateError {
    WASMDeocdeError,
    WATDecodeError,
    UnexpectedStreamEnd,
    Unknown,
} // enum ModuleCreateError

impl std::fmt::Display for ModuleCreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ModuleCreateError::UnexpectedStreamEnd => "unexpected stream end",
            ModuleCreateError::WASMDeocdeError => "WASM decode error",
            ModuleCreateError::WATDecodeError => "WAT decode error",
            ModuleCreateError::Unknown => "unknown",
        })
    }
}