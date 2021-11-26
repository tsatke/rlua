use std::io::Read;

use crate::file::header::Header;
use crate::file::LuaFileParseError;

#[derive(Debug)]
pub struct LuaString {
    pub data: Vec<u8>,
}

impl LuaString {
    pub fn parse(header: &Header, source: &mut impl Read) -> Result<String, LuaFileParseError> {
        let size = header.byte_order.read_u8(source)?;
        if size == 0 {
            return Ok("".to_owned());
        }

        let mut data = vec![0_u8; (size - 1) as usize];
        source
            .read_exact(&mut data)
            .or(Err(LuaFileParseError::UnexpectedEOF))?;

        let rust_string =
            String::from_utf8(data).or(Err(LuaFileParseError::InvalidBytesInString))?;

        Ok(rust_string)
    }
}
