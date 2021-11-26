use crate::file::LuaFileParseError;
use crate::opcode::{Mode, Op, Opcode};
use std::fmt::{Debug, Display, Formatter};

macro_rules! get_arg {
    ($instr:expr, $pos:expr, $size:expr) => {
        ($instr >> $pos) & mask_hi!($size, 0)
    };
}

macro_rules! mask_hi {
    (32, $pos:expr) => {
        (!0_u32)
    };
    ($num_hi:expr, $pos:expr) => {
        (!((!0_u32) << $num_hi) << $pos)
    };
}

macro_rules! mask_lo {
    ($num_lo:expr, $pos:expr) => {
        !mask_hi!($num_lo, $pos)
    };
}

const SIZE_OP: u8 = 6;
const SIZE_A: u8 = 8;
const SIZE_B: u8 = 9;
const SIZE_C: u8 = 9;
const SIZE_BX: u8 = SIZE_B + SIZE_C;
const SIZE_AX: u8 = SIZE_C + SIZE_B + SIZE_A;

const POS_OP: u8 = 0;
const POS_A: u8 = POS_OP + SIZE_OP;
const POS_C: u8 = POS_A + SIZE_A;
const POS_B: u8 = POS_C + SIZE_C;
const POS_BX: u8 = POS_C;
const POS_AX: u8 = POS_A;

const BIT_RK: u16 = 1 << (SIZE_B - 1);

pub struct ArgK(u16);

impl ArgK {
    pub fn is_constant(&self) -> bool {
        self.0 & BIT_RK == BIT_RK
    }

    pub fn index_k(&self) -> u8 {
        self.0 as u8 & !BIT_RK as u8
    }

    pub fn value(&self) -> u8 {
        self.0 as u8
    }
}

pub struct Instruction(u32);

impl TryFrom<u32> for Instruction {
    type Error = LuaFileParseError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let instr = Instruction(value);

        // check for valid instruction
        if get_arg!(value, POS_OP, SIZE_OP) > Op::ExtraArg as u32 {
            return Err(LuaFileParseError::InvalidInstruction);
        }

        Ok(instr)
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.get_op().name())?;
        match self.get_op().mode() {
            Mode::ABC => write!(
                f,
                "{} {} {}",
                self.get_a(),
                self.get_b().value(),
                self.get_c().value(),
            )?,
            Mode::ABx => write!(f, "{} {}", self.get_a(), self.get_bx())?,
            Mode::AsBx => write!(f, "{} {}", self.get_a(), self.get_sbx())?,
            Mode::Ax => write!(f, "{}", self.get_ax())?,
        }
        Ok(())
    }
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Instruction")
            .field("instr", &format!("0x{:08x}", self.0))
            .field("op", &self.get_op())
            .field("A", &self.get_a())
            .field("B", &self.get_b().value())
            .field("B_const", &self.get_b().is_constant())
            .field("C", &self.get_c().value())
            .field("C_const", &self.get_c().is_constant())
            .field("Bx", &self.get_bx())
            .field("sBx", &self.get_sbx())
            .finish()
    }
}

impl Opcode for Instruction {
    fn get_op(&self) -> Op {
        ((get_arg!(self.0, POS_OP, SIZE_OP)) as u8)
            .try_into()
            .unwrap()
    }

    fn get_a(&self) -> u8 {
        get_arg!(self.0, POS_A, SIZE_A) as u8
    }

    fn get_b(&self) -> ArgK {
        ArgK(get_arg!(self.0, POS_B, SIZE_B) as u16)
    }

    fn get_c(&self) -> ArgK {
        ArgK(get_arg!(self.0, POS_C, SIZE_C) as u16)
    }

    fn get_ax(&self) -> u32 {
        get_arg!(self.0, SIZE_AX, POS_AX)
    }

    fn get_bx(&self) -> u32 {
        get_arg!(self.0, SIZE_BX, POS_BX)
    }

    fn get_sbx(&self) -> i32 {
        self.get_bx() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Instruction;
    use crate::opcode::NUM_OP;

    #[test]
    fn test_opcodes() {
        for i in 0..NUM_OP {
            let instr = Instruction::try_from(i as u32).unwrap();
            assert_eq!(Op::try_from(i).unwrap(), instr.get_op());
        }

        let failure = Instruction::try_from(47);
        assert!(failure.is_err());
    }
}
