use rlua::file::byte_order::ByteOrder;
use rlua::file::chunk::Chunk;
use rlua::file::header::{ByteSize, Header};
use rlua::file::{Constant, LuaFile, LuaFileParseError};
use rlua::opcode::{Op, Opcode};
use std::fs::File;
use std::io::{BufReader, Cursor, Read};

#[test]
fn test_simple() {
    let mut os_file = File::open("tests/resources/simple.luac").unwrap();
    let mut rd = BufReader::new(os_file);
    let file_res = LuaFile::parse(&mut rd);
    let file = match file_res {
        Ok(f) => f,
        Err(e) => panic!("{:?}", e),
    };

    let header = file.header;
    assert_eq!(0x53, header.version);

    let main_chunk = file.main_chunk;
    assert_eq!("@simple.lua", main_chunk.name);
    assert_eq!(1, main_chunk.line_defined);
    assert_eq!(4, main_chunk.last_line_defined);
    assert_eq!(1, main_chunk.num_upvalues);
    assert_eq!(2, main_chunk.max_stack);

    // byte code
    assert_eq!(5, main_chunk.code.len());
    assert_eq!(Op::SetTabup, main_chunk.code[0].get_op());
    assert_eq!(Op::SetTabup, main_chunk.code[1].get_op());
    assert_eq!(Op::SetTabup, main_chunk.code[2].get_op());
    assert_eq!(Op::SetTabup, main_chunk.code[3].get_op());
    assert_eq!(Op::Return, main_chunk.code[4].get_op());

    // constant pool
    assert_eq!(8, main_chunk.constants.len());
    assert_eq!(Constant::String("a".to_owned()), main_chunk.constants[0]);
    assert_eq!(
        Constant::String("hello".to_owned()),
        main_chunk.constants[1]
    );
    assert_eq!(Constant::String("b".to_owned()), main_chunk.constants[2]);
    assert_eq!(Constant::IntegralNumber(1), main_chunk.constants[3]);
    assert_eq!(Constant::String("c".to_owned()), main_chunk.constants[4]);
    assert_eq!(Constant::FloatingNumber(1.5), main_chunk.constants[5]);
    assert_eq!(Constant::String("d".to_owned()), main_chunk.constants[6]);
    assert_eq!(Constant::String("bye".to_owned()), main_chunk.constants[7]);

    // source lines
    assert_eq!(vec![1, 2, 3, 4, 4], main_chunk.source_lines);

    // locals
    assert_eq!(0, main_chunk.locals.len());

    // upvalues
    assert_eq!(1, main_chunk.upvalue_names.len());
    assert_eq!("_ENV", main_chunk.upvalue_names[0]);
}
