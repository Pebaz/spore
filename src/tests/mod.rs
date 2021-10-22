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
                // Test that CC or CS have not effect on unconditional jump
                &[byte(0, 1, OpCode::JMP8)][..],
                &(-3i8).to_be_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "JMP8cc -3",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::JMP8)][..],
                &(-3i8).to_be_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "JMP8cs -3",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::JMP8)][..],
                &(-3i8).to_be_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "POPn R1",
        dis(opts, cur, &[OpCode::POPn.to(), 0b00000001])
    );

    assert_eq!(
        "PUSHn R1",
        dis(opts, cur, &[OpCode::PUSHn.to(), 0b00000001])
    );

    assert_eq!(
        "POPn @R1",
        dis(opts, cur, &[OpCode::POPn.to(), 0b00001001])
    );

    assert_eq!(
        "PUSHn @R1",
        dis(opts, cur, &[OpCode::PUSHn.to(), 0b00001001])
    );

    assert_eq!(
        "POPn R1 -3",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::POPn), 0b00000001][..],
                &(-3i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "PUSHn R1 -3",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::PUSHn), 0b00000001][..],
                &(-3i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "POPn @R1(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::POPn), 0b00001001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "PUSHn @R1(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::PUSHn), 0b00001001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!("POP32 R1", dis(opts, cur, &[OpCode::POP.to(), 0b00000001]));
    assert_eq!("PUSH32 R1", dis(opts, cur, &[OpCode::PUSH.to(), 0b00000001]));
    assert_eq!("POP32 @R1", dis(opts, cur, &[OpCode::POP.to(), 0b00001001]));
    assert_eq!("PUSH32 @R1", dis(opts, cur, &[OpCode::PUSH.to(), 0b00001001]));

    assert_eq!(
        "POP32 R1 -3",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::POP), 0b00000001][..],
                &(-3i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "PUSH32 R1 -3",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::PUSH), 0b00000001][..],
                &(-3i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "POP32 @R1(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::POP), 0b00001001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "PUSH32 @R1(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::PUSH), 0b00001001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "POP64 R1",
        dis(opts, cur, &[byte(0, 1, OpCode::POP), 0b00000001])
    );

    assert_eq!(
        "PUSH64 R1",
        dis(opts, cur, &[byte(0, 1, OpCode::PUSH), 0b00000001])
    );

    assert_eq!(
        "POP64 @R1",
        dis(opts, cur, &[byte(0, 1, OpCode::POP), 0b00001001])
    );

    assert_eq!(
        "PUSH64 @R1",
        dis(opts, cur, &[byte(0, 1, OpCode::PUSH), 0b00001001])
    );

    assert_eq!(
        "POP64 R1 -3",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::POP), 0b00000001][..],
                &(-3i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "PUSH64 R1 -3",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::PUSH), 0b00000001][..],
                &(-3i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "POP64 @R1(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::POP), 0b00001001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "PUSH64 @R1(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::PUSH), 0b00001001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!("CALL32 R1", dis(opts, cur, &[OpCode::CALL.to(), 0b00010001]));
    assert_eq!("CALL32a R1", dis(opts, cur, &[OpCode::CALL.to(), 0b00000001]));

    assert_eq!(
        "CALL32 R1 -3",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CALL), 0b00010001][..],
                &(-3i32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CALL32a R1 -3",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CALL), 0b00000001][..],
                &(-3i32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CALL32 @R1(-300, -300)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CALL), 0b00011001][..],
                &(2954019116u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!("CALL32 @R1", dis(opts, cur, &[OpCode::CALL.to(), 0b00011001]));

    assert_eq!(
        "CALL32a @R1(-300, -300)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CALL), 0b00001001][..],
                &(2954019116u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!("CALL32EX R1", dis(opts, cur, &[OpCode::CALL.to(), 0b00110001]));
    assert_eq!("CALL32EXa R1", dis(opts, cur, &[OpCode::CALL.to(), 0b00100001]));
    assert_eq!("CALL32EX @R1", dis(opts, cur, &[OpCode::CALL.to(), 0b00111001]));
    assert_eq!("CALL32EXa @R1", dis(opts, cur, &[OpCode::CALL.to(), 0b00101001]));

    assert_eq!(
        "CALL32EX R1 -3",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CALL), 0b00110001][..],
                &(-3i32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CALL32EXa R1 -3",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CALL), 0b00100001][..],
                &(-3i32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CALL32EX @R1(-300, -300)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CALL), 0b00111001][..],
                &(2954019116u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CALL32EXa @R1(-300, -300)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CALL), 0b00101001][..],
                &(2954019116u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CALL64EXa -3",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::CALL), 0b00110001][..],
                &(-3i64).to_le_bytes()[..],
            ].concat()
        )
    );
}
