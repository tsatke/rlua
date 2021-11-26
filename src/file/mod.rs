use std::io::Read;

use crate::file::string::LuaString;
use chunk::Chunk;
use header::Header;

pub mod byte_order;
pub mod chunk;
pub mod header;
mod string;

#[derive(Debug)]
pub enum LuaFileParseError {
    UnexpectedEOF,
    InvalidMagicValue,
    VersionMismatch,
    InvalidBytesInString,
    InvalidInstruction,
    InvalidConstantType,
    InvalidNumericConstantType,
    InvalidFloatingPointByteSize,
}

pub struct LuaFile {
    pub header: Header,
    pub main_chunk: Chunk,
}

impl LuaFile {
    pub fn parse(source: &mut impl Read) -> Result<LuaFile, LuaFileParseError> {
        let header = Header::parse(source)?;
        let main_chunk = Chunk::parse(&header, source)?;

        Ok(LuaFile { header, main_chunk })
    }
}

#[derive(Debug)]
pub struct VarArgInfo {}

#[derive(Debug, PartialEq)]
pub enum Constant {
    Nil,
    Boolean(bool),
    IntegralNumber(u64),
    FloatingNumber(f64),
    String(String),
}

#[derive(Debug)]
pub struct Local {
    varname: String,
    startpc: u64,
    endpc: u64,
}
