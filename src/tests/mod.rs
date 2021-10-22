use std::io::Cursor;
use super::*;

fn dis(options: &Options, cursor: &mut Cursor<Vec<u8>>, bytecode: &[u8]) -> String
{
    OpCode::disassemble(&options, cursor, &mut bytecode.into_iter().cloned());
    let disassembly = String::from_utf8(cursor.get_ref().clone()).unwrap();
    cursor.get_mut().fill(0);
    disassembly
}

#[test]
pub fn test_instruction_disassembly()
{
    let options = &Options { theme: None };
    let cursor = &mut Cursor::new(Vec::with_capacity(50));

    let disassembly = dis(options, cursor, &[OpCode::RET.to()]);
    assert_eq!("RET", disassembly);
}
