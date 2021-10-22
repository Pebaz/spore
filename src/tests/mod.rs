use std::io::Cursor;
use super::*;

fn dis(options: &Options, cursor: &mut Cursor<Vec<u8>>, bytecode: &[u8]) -> String
{
    OpCode::disassemble(&options, cursor, &mut bytecode.into_iter().cloned());

    let disassembly = String::from_utf8(cursor.get_ref().clone()).unwrap();
    cursor.get_mut().clear();
    cursor.set_position(0);

    // Remove last character since that will always be a "\n"
    let mut chars = disassembly.chars();
    chars.next_back();
    chars.as_str().to_string()
}

fn byte(bit8: u8, bit7: u8, op: OpCode) -> u8
{
    let mut byte = op.to();
    if bit8 > 0 { byte |= 1 << 7; }
    if bit7 > 0 { byte |= 1 << 6; }
    byte
}

#[test]
pub fn test_instruction_disassembly()
{
    let opts = &Options { theme: None };
    let cur = &mut Cursor::new(Vec::with_capacity(50));

    assert_eq!("RET", dis(opts, cur, &[OpCode::RET.to()]));
    assert_eq!(
        "STORESP R1, FLAGS",
        dis(opts, cur, &[OpCode::STORESP.to(), 0b00000001])
    );
    assert_eq!(
        "STORESP R1, IP",
        dis(opts, cur, &[OpCode::STORESP.to(), 0b00010001])
    );
    assert_eq!(
        "LOADSP FLAGS, R1",
        dis(opts, cur, &[OpCode::LOADSP.to(), 0b00010000])
    );
    assert_eq!(
        "LOADSP IP, R1",
        dis(opts, cur, &[OpCode::LOADSP.to(), 0b00010001])
    );
    assert_eq!("BREAK 3", dis(opts, cur, &[OpCode::BREAK.to(), 0b00000011]));

    assert_eq!(
        "JMP8 -3",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 0, OpCode::JMP8)][..],
                &(-3i8).to_be_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "JMP8 -3",
        dis(
            opts,
            cur,
            &[
                // &[OpCode::JMP8.to()][..],
                &[byte(0, 1, OpCode::JMP8)][..],
                &(-3i8).to_be_bytes()[..],
            ].concat()
        )
    );
}
