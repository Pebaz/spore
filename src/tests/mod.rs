use std::io::Cursor;
use super::*;

fn dis(options: &Options, cursor: &mut Cursor<Vec<u8>>, bytecode: &[u8]) -> String
{
    OpCode::disassemble(&options, cursor, &mut bytecode.into_iter().cloned());
    let disassembly = String::from_utf8(cursor.get_ref().clone()).unwrap();
    cursor.get_mut().fill(0);
    cursor.set_position(0);
    disassembly
}

#[test]
pub fn test_instruction_disassembly()
{
    let opts = &Options { theme: None };
    let cur = &mut Cursor::new(Vec::with_capacity(50));

    assert_eq!("RET", dis(opts, cur, &[OpCode::RET.to()]));
    assert_eq!("STORESP R1, FLAGS", dis(opts, cur, &[OpCode::STORESP.to(), 0b00000001]));
}
