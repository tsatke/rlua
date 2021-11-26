use crate::file::byte_order::ByteOrder::{BigEndian, LittleEndian};
use crate::file::header::ByteSize;
use crate::file::header::Header;
use crate::file::string::LuaString;
use crate::file::{Constant, Local, LuaFileParseError, VarArgInfo};
use crate::opcode::Instruction;
use crate::{read_bytes, read_integral, read_lua_float, read_lua_int};
use std::io::Read;

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

        Ok(Chunk {
            name,
            line_defined: 0,
            last_line_defined: 0,
            num_upvalues,
            num_params: 0,
            vararg_info: None,
            max_stack,
            code,
            constants,
            prototypes: vec![],
            source_lines: vec![],
            locals: vec![],
            upvalue_names: vec![],
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
                        0 => Constant::FloatingNumber(read_lua_float!(header, source)),
                        1 => Constant::IntegralNumber(read_lua_int!(header, source)),
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
}
