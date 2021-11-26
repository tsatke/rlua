use crate::file::LuaFileParseError;
use crate::instruction::ArgK;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt::{Debug, Display, Formatter};
use std::ptr::write;

#[derive(Debug, Eq, PartialEq)]
pub enum Mode {
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

pub const NUM_OP: u8 = Op::ExtraArg as u8 + 1;

impl Op {
    pub fn mode(&self) -> Mode {
        match self {
            Op::Move => Mode::ABC,
            Op::LoadK => Mode::ABx,
            Op::LoadKx => Mode::ABx,
            Op::LoadBool => Mode::ABC,
            Op::LoadNil => Mode::ABC,
            Op::GetUpval => Mode::ABC,
            Op::GetTabup => Mode::ABC,
            Op::GetTable => Mode::ABC,
            Op::SetTabup => Mode::ABC,
            Op::SetUpval => Mode::ABC,
            Op::SetTable => Mode::ABC,
            Op::NewTable => Mode::ABC,
            Op::LuaSelf => Mode::ABC,
            Op::Add => Mode::ABC,
            Op::Sub => Mode::ABC,
            Op::Mul => Mode::ABC,
            Op::Mod => Mode::ABC,
            Op::Pow => Mode::ABC,
            Op::Div => Mode::ABC,
            Op::IDiv => Mode::ABC,
            Op::BAnd => Mode::ABC,
            Op::BOr => Mode::ABC,
            Op::BXor => Mode::ABC,
            Op::Shl => Mode::ABC,
            Op::Shr => Mode::ABC,
            Op::Unm => Mode::ABC,
            Op::BNot => Mode::ABC,
            Op::Not => Mode::ABC,
            Op::Len => Mode::ABC,
            Op::Concat => Mode::ABC,
            Op::Jmp => Mode::AsBx,
            Op::Eq => Mode::ABC,
            Op::Lt => Mode::ABC,
            Op::Le => Mode::ABC,
            Op::Test => Mode::ABC,
            Op::TestSet => Mode::ABC,
            Op::Call => Mode::ABC,
            Op::Tailcall => Mode::ABC,
            Op::Return => Mode::ABC,
            Op::ForLoop => Mode::AsBx,
            Op::ForPrep => Mode::AsBx,
            Op::TForCall => Mode::ABC,
            Op::TForLoop => Mode::AsBx,
            Op::SetList => Mode::ABC,
            Op::Closure => Mode::ABx,
            Op::VarArg => Mode::ABC,
            Op::ExtraArg => Mode::Ax,
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

pub trait Opcode {
    fn get_op(&self) -> Op;
    fn get_a(&self) -> u8;
    fn get_b(&self) -> ArgK;
    fn get_c(&self) -> ArgK;
    fn get_ax(&self) -> u32;
    fn get_bx(&self) -> u32;
    fn get_sbx(&self) -> i32;
}
