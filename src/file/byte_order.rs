use crate::file::LuaFileParseError;
use crate::read_bytes;
use std::fmt::{Display, Formatter};
use std::io::Read;

#[macro_export]
macro_rules! read_bytes {
    ($source:expr, $count:expr) => {{
        let mut buf = [0_u8; $count];
        $source
            .read_exact(&mut buf)
            .or(Err(LuaFileParseError::UnexpectedEOF))?;
        buf
    }};
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum ByteOrder {
    BigEndian,
    LittleEndian,
}

impl Display for ByteOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ByteOrder::BigEndian => write!(f, "big endian"),
            ByteOrder::LittleEndian => write!(f, "little endian"),
        }
    }
}

impl ByteOrder {
    pub fn read_u8(&self, source: &mut impl Read) -> Result<u8, LuaFileParseError> {
        Ok(read_bytes!(source, 1)[0])
    }

    pub fn read_u16(&self, source: &mut impl Read) -> Result<u16, LuaFileParseError> {
        Ok(match self {
            ByteOrder::BigEndian => u16::from_be_bytes(read_bytes!(source, 2)),
            ByteOrder::LittleEndian => u16::from_le_bytes(read_bytes!(source, 2)),
        })
    }

    pub fn read_u32(&self, source: &mut impl Read) -> Result<u32, LuaFileParseError> {
        Ok(match self {
            ByteOrder::BigEndian => u32::from_be_bytes(read_bytes!(source, 4)),
            ByteOrder::LittleEndian => u32::from_le_bytes(read_bytes!(source, 4)),
        })
    }

    pub fn read_u64(&self, source: &mut impl Read) -> Result<u64, LuaFileParseError> {
        Ok(match self {
            ByteOrder::BigEndian => u64::from_be_bytes(read_bytes!(source, 8)),
            ByteOrder::LittleEndian => u64::from_le_bytes(read_bytes!(source, 8)),
        })
    }

    pub fn read_f32(&self, source: &mut impl Read) -> Result<f32, LuaFileParseError> {
        Ok(match self {
            ByteOrder::BigEndian => f32::from_be_bytes(read_bytes!(source, 4)),
            ByteOrder::LittleEndian => f32::from_le_bytes(read_bytes!(source, 4)),
        })
    }

    pub fn read_f64(&self, source: &mut impl Read) -> Result<f64, LuaFileParseError> {
        Ok(match self {
            ByteOrder::BigEndian => f64::from_be_bytes(read_bytes!(source, 8)),
            ByteOrder::LittleEndian => f64::from_le_bytes(read_bytes!(source, 8)),
        })
    }
}
