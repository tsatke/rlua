use crate::file::LuaFileParseError;
use num_enum::TryFromPrimitive;
use std::fmt::{Debug, Display, Formatter};
use std::ptr::write;

#[derive(Debug, Eq, PartialEq)]
pub enum OpMode {
    ABC,
    ABx,
    AsBx,
    Ax,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Op {
    Move,
    LoadK,
    LoadKx,
    LoadBool,
    LoadNil,
    GetUpval,
    GetTabup,
    GetTable,
    SetTabup,
    SetUpval,
    SetTable,
    NewTable,
    LuaSelf,
    Add,
    Sub,
    Mul,
    Mod,
    Pow,
    Div,
    IDiv,
    BAnd,
    BOr,
    BXor,
    Shl,
    Shr,
    Unm,
    BNot,
    Not,
    Len,
    Concat,
    Jmp,
    Eq,
    Lt,
    Le,
    Test,
    TestSet,
    Call,
    Tailcall,
    Return,
    ForLoop,
    ForPrep,
    TForCall,
    TForLoop,
    SetList,
    Closure,
    VarArg,
    ExtraArg,
}

impl Op {
    pub fn mode(&self) -> OpMode {
        match self {
            Op::Move => OpMode::ABC,
            Op::LoadK => OpMode::ABx,
            Op::LoadKx => OpMode::ABx,
            Op::LoadBool => OpMode::ABC,
            Op::LoadNil => OpMode::ABC,
            Op::GetUpval => OpMode::ABC,
            Op::GetTabup => OpMode::ABC,
            Op::GetTable => OpMode::ABC,
            Op::SetTabup => OpMode::ABC,
            Op::SetUpval => OpMode::ABC,
            Op::SetTable => OpMode::ABC,
            Op::NewTable => OpMode::ABC,
            Op::LuaSelf => OpMode::ABC,
            Op::Add => OpMode::ABC,
            Op::Sub => OpMode::ABC,
            Op::Mul => OpMode::ABC,
            Op::Mod => OpMode::ABC,
            Op::Pow => OpMode::ABC,
            Op::Div => OpMode::ABC,
            Op::IDiv => OpMode::ABC,
            Op::BAnd => OpMode::ABC,
            Op::BOr => OpMode::ABC,
            Op::BXor => OpMode::ABC,
            Op::Shl => OpMode::ABC,
            Op::Shr => OpMode::ABC,
            Op::Unm => OpMode::ABC,
            Op::BNot => OpMode::ABC,
            Op::Not => OpMode::ABC,
            Op::Len => OpMode::ABC,
            Op::Concat => OpMode::ABC,
            Op::Jmp => OpMode::AsBx,
            Op::Eq => OpMode::ABC,
            Op::Lt => OpMode::ABC,
            Op::Le => OpMode::ABC,
            Op::Test => OpMode::ABC,
            Op::TestSet => OpMode::ABC,
            Op::Call => OpMode::ABC,
            Op::Tailcall => OpMode::ABC,
            Op::Return => OpMode::ABC,
            Op::ForLoop => OpMode::AsBx,
            Op::ForPrep => OpMode::AsBx,
            Op::TForCall => OpMode::ABC,
            Op::TForLoop => OpMode::AsBx,
            Op::SetList => OpMode::ABC,
            Op::Closure => OpMode::ABx,
            Op::VarArg => OpMode::ABC,
            Op::ExtraArg => OpMode::Ax,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Op::Move => "Move",
            Op::LoadK => "LoadK",
            Op::LoadKx => "LoadKx",
            Op::LoadBool => "LoadBool",
            Op::LoadNil => "LoadNil",
            Op::GetUpval => "GetUpval",
            Op::GetTabup => "GetTabup",
            Op::GetTable => "GetTable",
            Op::SetTabup => "SetTabup",
            Op::SetUpval => "SetUpval",
            Op::SetTable => "SetTable",
            Op::NewTable => "NewTable",
            Op::LuaSelf => "LuaSelf",
            Op::Add => "Add",
            Op::Sub => "Sub",
            Op::Mul => "Mul",
            Op::Mod => "Mod",
            Op::Pow => "Pow",
            Op::Div => "Div",
            Op::IDiv => "IdiV",
            Op::BAnd => "BAnd",
            Op::BOr => "BOr",
            Op::BXor => "BXor",
            Op::Shl => "Shl",
            Op::Shr => "Shr",
            Op::Unm => "Unm",
            Op::BNot => "BNot",
            Op::Not => "Not",
            Op::Len => "Len",
            Op::Concat => "Concat",
            Op::Jmp => "Jmp",
            Op::Eq => "Eq",
            Op::Lt => "Lt",
            Op::Le => "Le",
            Op::Test => "Test",
            Op::TestSet => "TestSet",
            Op::Call => "Call",
            Op::Tailcall => "Tailcall",
            Op::Return => "Return",
            Op::ForLoop => "ForLoop",
            Op::ForPrep => "ForPrep",
            Op::TForCall => "TForCall",
            Op::TForLoop => "TForLoop",
            Op::SetList => "SetList",
            Op::Closure => "Closure",
            Op::VarArg => "VarArg",
            Op::ExtraArg => "ExtraArg",
        }
    }
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

pub trait Opcode {
    fn get_op(&self) -> Op;
    fn get_a(&self) -> u8;
    fn get_b(&self) -> ArgK;
    fn get_c(&self) -> ArgK;
    fn get_ax(&self) -> u32;
    fn get_bx(&self) -> u32;
    fn get_sbx(&self) -> i32;
}

pub struct Instruction(u32);

impl TryFrom<u32> for Instruction {
    type Error = LuaFileParseError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let instr = Instruction(value);

        // check for valid instruction
        if instr.get_op() > Op::ExtraArg {
            return Err(LuaFileParseError::InvalidInstruction);
        }

        Ok(instr)
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.get_op().name())?;
        match self.get_op().mode() {
            OpMode::ABC => write!(
                f,
                "{} {} {}",
                self.get_a(),
                self.get_b().value(),
                self.get_c().value(),
            )?,
            OpMode::ABx => write!(f, "{} {}", self.get_a(), self.get_bx())?,
            OpMode::AsBx => write!(f, "{} {}", self.get_a(), self.get_sbx())?,
            OpMode::Ax => write!(f, "{}", self.get_ax())?,
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

macro_rules! get_arg {
    ($instr:expr, $pos:expr, $size:expr) => {
        ($instr >> $pos) & mask_hi!($size, 0)
    };
}

impl Opcode for Instruction {
    fn get_op(&self) -> Op {
        (((self.0 >> POS_OP) & mask_hi!(SIZE_OP, POS_OP)) as u8)
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
