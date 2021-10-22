use std::io::Cursor;
use super::*;

// fn dis<T>(options: &Options, cursor: &mut Cursor<T>, bytecode: &[u8]) -> String
// where std::io::Cursor<T>: std::io::Write
// {
//     OpCode::disassemble(&options, cursor, &mut bytecode.into_iter().cloned());
//     let disassembly = String::from_utf8(cursor.get_ref().clone()).unwrap();
//     cursor.get_mut().fill(0);
//     disassembly
// }

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
    // 1. Pass bytecode to be disassembled
    // 2. Capture disassembly text
    // 3. Compare against static string

    // OpCode::disassemble(&mut bytes).is_none()
    // assert_eq!(1, 1);

    let options = &Options
    {
        theme: None,
    };
    let cursor = &mut Cursor::new(Vec::with_capacity(50));

    // OpCode::disassemble(&options, cursor, &mut vec![OpCode::RET.to()].into_iter());
    // let disassembly = String::from_utf8(cursor.get_ref().clone()).unwrap()
    // cursor.get_mut().fill(0);
    let disassembly = dis(options, cursor, &[OpCode::RET.to()]);

    assert_eq!("RET", disassembly);

    // ? cursor.set_position(0);






    // write(&mut cursor);

    // println!("-> {}", String::from_utf8(cursor.get_ref().clone()).unwrap());

    // cursor.get_mut().fill(0);

    // cursor.set_position(0);

    // write(&mut cursor);

    // println!("-> {}", String::from_utf8(cursor.get_ref().clone()).unwrap());
}
