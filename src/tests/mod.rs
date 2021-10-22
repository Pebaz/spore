use std::io::Cursor;
use super::*;

#[test]
pub fn test_instruction_disassembly()
{
    // 1. Pass bytecode to be disassembled
    // 2. Capture disassembly text
    // 3. Compare against static string

    // OpCode::disassemble(&mut bytes).is_none()
    // assert_eq!(1, 1);

    let options = Options
    {
        theme: None,
    };
    let cursor = &mut Cursor::new(Vec::with_capacity(50));

    OpCode::disassemble(&options, cursor, &mut vec![OpCode::RET.to()].into_iter());

    println!("-> {}", String::from_utf8(cursor.get_ref().clone()).unwrap());

    cursor.get_mut().fill(0);
    // ? cursor.set_position(0);






    // write(&mut cursor);

    // println!("-> {}", String::from_utf8(cursor.get_ref().clone()).unwrap());

    // cursor.get_mut().fill(0);

    // cursor.set_position(0);

    // write(&mut cursor);

    // println!("-> {}", String::from_utf8(cursor.get_ref().clone()).unwrap());
}
