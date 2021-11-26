use std::io::Read;

use crate::file::byte_order::ByteOrder::{BigEndian, LittleEndian};
use crate::file::header::ByteSize;
use crate::file::header::Header;
use crate::file::string::LuaString;
use crate::file::{Constant, Local, LuaFileParseError, VarArgInfo};
use crate::instruction::Instruction;
use crate::{
    read_bytes, read_integral, read_lua_int, read_lua_number_float, read_lua_number_integral,
};

#[derive(Debug)]
pub struct Chunk {
    pub name: String,
    pub line_defined: u64,
    pub last_line_defined: u64,
    pub num_upvalues: u8,
    pub num_params: u8,
    pub vararg_info: Option<VarArgInfo>,
    pub max_stack: u8,
    pub code: Vec<Instruction>,
    pub constants: Vec<Constant>,
    pub prototypes: Vec<Chunk>,
    pub source_lines: Vec<u64>,
    pub locals: Vec<Local>,
    pub upvalue_names: Vec<String>,
}

impl Chunk {
    pub fn parse(header: &Header, source: &mut impl Read) -> Result<Chunk, LuaFileParseError> {
        let num_upvalues = header.byte_order.read_u8(source)?;
        let name = LuaString::parse(header, source)?;

        let _ = read_bytes!(source, 10); // FIXME: don't know what that is

        let max_stack = header.byte_order.read_u8(source)?;
        let num_instructions = header.byte_order.read_u32(source)?;
        let mut code = Vec::with_capacity(num_instructions as usize);
        for _ in 0..num_instructions {
            let instruction = Instruction::try_from(header.byte_order.read_u32(source)?)?;
            code.push(instruction);
        }

        let constants = Chunk::parse_constants(&header, source)?;

        let _ = read_bytes!(source, 10); // FIXME: don't know what that is

        let source_lines = Chunk::parse_source_lines(&header, source)?;
        let locals = Chunk::parse_locals(&header, source)?;
        let upvalue_names = Chunk::parse_upvalues(&header, source)?;

        Ok(Chunk {
            name,
            line_defined: source_lines[0],
            last_line_defined: source_lines[source_lines.len() - 1],
            num_upvalues,
            num_params: 0,
            vararg_info: None,
            max_stack,
            code,
            constants,
            prototypes: vec![],
            source_lines,
            locals,
            upvalue_names,
        })
    }

    fn parse_constants(
        header: &Header,
        source: &mut impl Read,
    ) -> Result<Vec<Constant>, LuaFileParseError> {
        let sizek = header.byte_order.read_u32(source)?;
        let mut constants = Vec::with_capacity(sizek as usize);

        for _ in 0..sizek {
            let constant_type = header.byte_order.read_u8(source)?;
            let c = match constant_type & 0xf {
                0 => Constant::Nil,
                1 => Constant::Boolean(header.byte_order.read_u8(source)? != 0),
                3 => {
                    let numeric_constant_type = (constant_type & 0xf0) >> 4;
                    match numeric_constant_type {
                        0 => Constant::FloatingNumber(read_lua_number_float!(header, source)),
                        1 => Constant::IntegralNumber(read_lua_number_integral!(header, source)),
                        _ => return Err(LuaFileParseError::InvalidNumericConstantType),
                    }
                }
                4 => Constant::String(LuaString::parse(&header, source)?),
                _ => return Err(LuaFileParseError::InvalidConstantType),
            };

            constants.push(c);
        }

        Ok(constants)
    }

    fn parse_upvalues(
        header: &Header,
        source: &mut impl Read,
    ) -> Result<Vec<String>, LuaFileParseError> {
        let num_upvalues = read_lua_int!(header, source);
        let mut upvalue_names = Vec::with_capacity(num_upvalues as usize);
        for _ in 0..num_upvalues {
            upvalue_names.push(LuaString::parse(&header, source)?);
        }
        Ok(upvalue_names)
    }

    fn parse_source_lines(
        header: &Header,
        source: &mut impl Read,
    ) -> Result<Vec<u64>, LuaFileParseError> {
        let num_source_lines = read_lua_int!(header, source);
        let mut source_lines = Vec::with_capacity(num_source_lines as usize);
        for _ in 0..num_source_lines {
            source_lines.push(read_lua_int!(header, source));
        }
        Ok(source_lines)
    }

    fn parse_locals(
        header: &Header,
        source: &mut impl Read,
    ) -> Result<Vec<Local>, LuaFileParseError> {
        let num_locals = read_lua_int!(header, source);
        let mut locals = Vec::with_capacity(num_locals as usize);
        for _ in 0..num_locals {
            let varname = LuaString::parse(&header, source)?;
            let startpc = read_lua_int!(header, source);
            let endpc = read_lua_int!(header, source);

            locals.push(Local {
                varname,
                startpc,
                endpc,
            });
        }
        Ok(locals)
    }
}
