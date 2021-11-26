use crate::file::byte_order::ByteOrder;
use crate::file::LuaFileParseError;
use crate::{debugln, read_bytes};
use num_enum::TryFromPrimitive;
use std::io::Read;

pub struct Header {
    pub version: u8,
    pub byte_order: ByteOrder,
    pub int_size: ByteSize,
    pub ptr_size: ByteSize,
    pub instruction_size: ByteSize,
    pub size_number_integral: ByteSize,
    pub size_number_float: ByteSize,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum ByteSize {
    _8 = 8,
    _4 = 4,
    _2 = 2,
    _1 = 1,
}

#[macro_export]
macro_rules! read_integral {
    ($header:expr, $source:expr, $byte_size:expr) => {
        match $byte_size {
            ByteSize::_8 => $header.byte_order.read_u64($source)? as u64,
            ByteSize::_4 => $header.byte_order.read_u32($source)? as u64,
            ByteSize::_2 => $header.byte_order.read_u16($source)? as u64,
            ByteSize::_1 => $header.byte_order.read_u8($source)? as u64,
        }
    };
}

#[macro_export]
macro_rules! read_lua_size_t {
    ($header:expr, $source:expr) => {{
        read_integral!($header, $source, $header.ptr_size)
    }};
}

#[macro_export]
macro_rules! read_lua_int {
    ($header:expr, $source:expr) => {{
        read_integral!($header, $source, $header.int_size)
    }};
}

#[macro_export]
macro_rules! read_lua_number_integral {
    ($header:expr, $source:expr) => {{
        read_integral!($header, $source, $header.size_number_integral)
    }};
}

#[macro_export]
macro_rules! read_lua_number_float {
    ($header:expr, $source:expr) => {{
        match $header.size_number_float {
            ByteSize::_8 => $header.byte_order.read_f64($source)? as f64,
            ByteSize::_4 => $header.byte_order.read_f32($source)? as f64,
            _ => return Err(LuaFileParseError::InvalidFloatingPointByteSize),
        }
    }};
}

impl Header {
    pub fn parse(source: &mut impl Read) -> Result<Header, LuaFileParseError> {
        if read_bytes!(source, 4) != "\x1bLua".as_bytes() {
            return Err(LuaFileParseError::InvalidMagicValue);
        }

        let version = read_bytes!(source, 1)[0];
        if version != 0x53 {
            return Err(LuaFileParseError::VersionMismatch);
        }

        let _format = read_bytes!(source, 1);
        let big_endian = read_bytes!(source, 1)[0] == 0;

        let byte_order = match big_endian {
            true => ByteOrder::BigEndian,
            false => ByteOrder::LittleEndian,
        };

        let _ = read_bytes!(source, 5); // more magic

        let int_size = byte_order.read_u8(source)?;
        let ptr_size = byte_order.read_u8(source)?;
        let instruction_size = byte_order.read_u8(source)?;
        if instruction_size != 4 {
            unimplemented!("only 4 byte instructions supported")
        }

        let size_number_integral = byte_order.read_u8(source)?;
        let size_number_float = byte_order.read_u8(source)?;

        // even more magic
        let _ = read_bytes!(source, 8);
        let _ = read_bytes!(source, 8);

        Ok(Header {
            version,
            byte_order,
            int_size: int_size.try_into().unwrap(),
            ptr_size: ptr_size.try_into().unwrap(),
            instruction_size: instruction_size.try_into().unwrap(),
            size_number_integral: size_number_integral.try_into().unwrap(),
            size_number_float: size_number_float.try_into().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;
    use std::io::Cursor;

    #[test]
    fn test_header_parse_simple() {
        let header = hex!(
            "
            1b 4c 75 61 53 00 19 93 0d 0a 1a 0a 04 08 04 08
            08 78 56 00 00 00 00 00 00 00 00 00 00 00 28 77
            40
            "
        );
        let mut rd = Cursor::new(header);
        let result = Header::parse(&mut rd);
        assert!(result.is_ok());
    }
}
