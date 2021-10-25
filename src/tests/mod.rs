use std::io::Cursor;
use super::*;

fn dis(options: &Options, cursor: &mut Cursor<Vec<u8>>, bytecode: &[u8]) -> String
{
    let result = OpCode::disassemble(
        &options,
        cursor,
        &mut bytecode.into_iter().cloned()
    );

    match result
    {
        Ok(_) => assert!(true),
        Err(msg) => assert!(false, "{}", msg.to_string()),
    }

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
                &(-3i8).to_le_bytes()[..],
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
                &(-3i8).to_le_bytes()[..],
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
                &(-3i8).to_le_bytes()[..],
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
                &(-3i8).to_le_bytes()[..],
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

    assert_eq!(
        "JMP32 R1  ;; Absolute Address",
        dis(opts, cur, &[byte(0, 0, OpCode::JMP), 0b00000001])
    );

    assert_eq!(
        "JMP32cc R1  ;; Absolute Address",
        dis(opts, cur, &[byte(0, 0, OpCode::JMP), 0b10000001])
    );

    assert_eq!(
        "JMP32cs R1  ;; Absolute Address",
        dis(opts, cur, &[byte(0, 0, OpCode::JMP), 0b11000001])
    );

    assert_eq!(
        "JMP32 R1  ;; Relative Address",
        dis(opts, cur, &[byte(0, 0, OpCode::JMP), 0b00010001])
    );

    assert_eq!(
        "JMP32cc R1  ;; Relative Address",
        dis(opts, cur, &[byte(0, 0, OpCode::JMP), 0b10010001])
    );

    assert_eq!(
        "JMP32cs R1  ;; Relative Address",
        dis(opts, cur, &[byte(0, 0, OpCode::JMP), 0b11010001])
    );

    assert_eq!(
        "JMP32 @R1  ;; Relative Address",
        dis(opts, cur, &[byte(0, 0, OpCode::JMP), 0b00011001])
    );

    assert_eq!(
        "JMP32 @R1  ;; Absolute Address",
        dis(opts, cur, &[byte(0, 0, OpCode::JMP), 0b00001001])
    );

    assert_eq!(
        "MOVb R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::MOVbw), 0b00100001])
    );

    assert_eq!(
        "MOVw R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::MOVww), 0b00100001])
    );

    assert_eq!(
        "MOVd R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::MOVdw), 0b00100001])
    );

    assert_eq!(
        "MOVq R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::MOVqw), 0b00100001])
    );

    assert_eq!(
        "MOVb @R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::MOVbw), 0b00101001])
    );

    assert_eq!(
        "MOVw @R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::MOVww), 0b00101001])
    );

    assert_eq!(
        "MOVd @R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::MOVdw), 0b00101001])
    );

    assert_eq!(
        "MOVq @R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::MOVqw), 0b00101001])
    );

    assert_eq!(
        "MOVb R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::MOVbw), 0b10100001])
    );

    assert_eq!(
        "MOVw R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::MOVww), 0b10100001])
    );

    assert_eq!(
        "MOVd R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::MOVdw), 0b10100001])
    );

    assert_eq!(
        "MOVq R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::MOVqw), 0b10100001])
    );

    assert_eq!(
        "MOVb @R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::MOVbw), 0b10101001])
    );

    assert_eq!(
        "MOVw @R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::MOVww), 0b10101001])
    );

    assert_eq!(
        "MOVd @R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::MOVdw), 0b10101001])
    );

    assert_eq!(
        "MOVq @R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::MOVqw), 0b10101001])
    );

    assert_eq!(
        "ADD32 R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::ADD), 0b00100001])
    );

    assert_eq!(
        "ADD32 @R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::ADD), 0b00101001])
    );

    assert_eq!(
        "ADD32 R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::ADD), 0b10100001])
    );

    assert_eq!(
        "ADD32 @R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::ADD), 0b10101001])
    );

    assert_eq!(
        "ADD64 R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::ADD), 0b00100001])
    );

    assert_eq!(
        "ADD64 @R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::ADD), 0b00101001])
    );

    assert_eq!(
        "ADD64 R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::ADD), 0b10100001])
    );

    assert_eq!(
        "ADD64 @R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::ADD), 0b10101001])
    );

    assert_eq!(
        "AND32 R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::AND), 0b00100001])
    );

    assert_eq!(
        "AND32 @R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::AND), 0b00101001])
    );

    assert_eq!(
        "AND32 R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::AND), 0b10100001])
    );

    assert_eq!(
        "AND32 @R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::AND), 0b10101001])
    );

    assert_eq!(
        "AND64 R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::AND), 0b00100001])
    );

    assert_eq!(
        "AND64 @R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::AND), 0b00101001])
    );

    assert_eq!(
        "AND64 R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::AND), 0b10100001])
    );

    assert_eq!(
        "AND64 @R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::AND), 0b10101001])
    );

    assert_eq!(
        "ASHR32 R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::ASHR), 0b00100001])
    );

    assert_eq!(
        "ASHR32 @R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::ASHR), 0b00101001])
    );

    assert_eq!(
        "ASHR32 R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::ASHR), 0b10100001])
    );

    assert_eq!(
        "ASHR32 @R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::ASHR), 0b10101001])
    );

    assert_eq!(
        "ASHR64 R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::ASHR), 0b00100001])
    );

    assert_eq!(
        "ASHR64 @R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::ASHR), 0b00101001])
    );

    assert_eq!(
        "ASHR64 R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::ASHR), 0b10100001])
    );

    assert_eq!(
        "ASHR64 @R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::ASHR), 0b10101001])
    );

    assert_eq!(
        "DIV32 R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::DIV), 0b00100001])
    );

    assert_eq!(
        "DIV32 @R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::DIV), 0b00101001])
    );

    assert_eq!(
        "DIV32 R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::DIV), 0b10100001])
    );

    assert_eq!(
        "DIV32 @R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::DIV), 0b10101001])
    );

    assert_eq!(
        "DIV64 R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::DIV), 0b00100001])
    );

    assert_eq!(
        "DIV64 @R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::DIV), 0b00101001])
    );

    assert_eq!(
        "DIV64 R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::DIV), 0b10100001])
    );

    assert_eq!(
        "DIV64 @R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::DIV), 0b10101001])
    );

    assert_eq!(
        "DIVU32 R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::DIVU), 0b00100001])
    );

    assert_eq!(
        "DIVU32 @R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::DIVU), 0b00101001])
    );

    assert_eq!(
        "DIVU32 R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::DIVU), 0b10100001])
    );

    assert_eq!(
        "DIVU32 @R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::DIVU), 0b10101001])
    );

    assert_eq!(
        "DIVU64 R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::DIVU), 0b00100001])
    );

    assert_eq!(
        "DIVU64 @R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::DIVU), 0b00101001])
    );

    assert_eq!(
        "DIVU64 R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::DIVU), 0b10100001])
    );

    assert_eq!(
        "DIVU64 @R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::DIVU), 0b10101001])
    );

    assert_eq!(
        "EXTNDB32 R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::EXTNDB), 0b00100001])
    );

    assert_eq!(
        "EXTNDB32 @R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::EXTNDB), 0b00101001])
    );

    assert_eq!(
        "EXTNDB32 R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::EXTNDB), 0b10100001])
    );

    assert_eq!(
        "EXTNDB32 @R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::EXTNDB), 0b10101001])
    );

    assert_eq!(
        "EXTNDB64 R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::EXTNDB), 0b00100001])
    );

    assert_eq!(
        "EXTNDB64 @R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::EXTNDB), 0b00101001])
    );

    assert_eq!(
        "EXTNDB64 R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::EXTNDB), 0b10100001])
    );

    assert_eq!(
        "EXTNDB64 @R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::EXTNDB), 0b10101001])
    );

    assert_eq!(
        "EXTNDD32 R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::EXTNDD), 0b00100001])
    );

    assert_eq!(
        "EXTNDD32 @R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::EXTNDD), 0b00101001])
    );

    assert_eq!(
        "EXTNDD32 R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::EXTNDD), 0b10100001])
    );

    assert_eq!(
        "EXTNDD32 @R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::EXTNDD), 0b10101001])
    );

    assert_eq!(
        "EXTNDD64 R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::EXTNDD), 0b00100001])
    );

    assert_eq!(
        "EXTNDD64 @R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::EXTNDD), 0b00101001])
    );

    assert_eq!(
        "EXTNDD64 R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::EXTNDD), 0b10100001])
    );

    assert_eq!(
        "EXTNDD64 @R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::EXTNDD), 0b10101001])
    );

    assert_eq!(
        "EXTNDW32 R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::EXTNDW), 0b00100001])
    );

    assert_eq!(
        "EXTNDW32 @R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::EXTNDW), 0b00101001])
    );

    assert_eq!(
        "EXTNDW32 R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::EXTNDW), 0b10100001])
    );

    assert_eq!(
        "EXTNDW32 @R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::EXTNDW), 0b10101001])
    );

    assert_eq!(
        "EXTNDW64 R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::EXTNDW), 0b00100001])
    );

    assert_eq!(
        "EXTNDW64 @R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::EXTNDW), 0b00101001])
    );

    assert_eq!(
        "EXTNDW64 R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::EXTNDW), 0b10100001])
    );

    assert_eq!(
        "EXTNDW64 @R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::EXTNDW), 0b10101001])
    );

    assert_eq!(
        "MOD32 R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::MOD), 0b00100001])
    );

    assert_eq!(
        "MOD32 @R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::MOD), 0b00101001])
    );

    assert_eq!(
        "MOD32 R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::MOD), 0b10100001])
    );

    assert_eq!(
        "MOD32 @R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::MOD), 0b10101001])
    );

    assert_eq!(
        "MOD64 R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::MOD), 0b00100001])
    );

    assert_eq!(
        "MOD64 @R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::MOD), 0b00101001])
    );

    assert_eq!(
        "MOD64 R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::MOD), 0b10100001])
    );

    assert_eq!(
        "MOD64 @R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::MOD), 0b10101001])
    );

    assert_eq!(
        "MODU32 R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::MODU), 0b00100001])
    );

    assert_eq!(
        "MODU32 @R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::MODU), 0b00101001])
    );

    assert_eq!(
        "MODU32 R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::MODU), 0b10100001])
    );

    assert_eq!(
        "MODU32 @R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::MODU), 0b10101001])
    );

    assert_eq!(
        "MODU64 R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::MODU), 0b00100001])
    );

    assert_eq!(
        "MODU64 @R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::MODU), 0b00101001])
    );

    assert_eq!(
        "MODU64 R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::MODU), 0b10100001])
    );

    assert_eq!(
        "MODU64 @R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::MODU), 0b10101001])
    );

    assert_eq!(
        "SHL32 R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::SHL), 0b00100001])
    );

    assert_eq!(
        "SHL32 @R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::SHL), 0b00101001])
    );

    assert_eq!(
        "SHL32 R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::SHL), 0b10100001])
    );

    assert_eq!(
        "SHL32 @R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::SHL), 0b10101001])
    );

    assert_eq!(
        "SHL64 R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::SHL), 0b00100001])
    );

    assert_eq!(
        "SHL64 @R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::SHL), 0b00101001])
    );

    assert_eq!(
        "SHL64 R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::SHL), 0b10100001])
    );

    assert_eq!(
        "SHL64 @R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::SHL), 0b10101001])
    );

    assert_eq!(
        "SHR32 R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::SHR), 0b00100001])
    );

    assert_eq!(
        "SHR32 @R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::SHR), 0b00101001])
    );

    assert_eq!(
        "SHR32 R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::SHR), 0b10100001])
    );

    assert_eq!(
        "SHR32 @R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::SHR), 0b10101001])
    );

    assert_eq!(
        "SHR64 R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::SHR), 0b00100001])
    );

    assert_eq!(
        "SHR64 @R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::SHR), 0b00101001])
    );

    assert_eq!(
        "SHR64 R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::SHR), 0b10100001])
    );

    assert_eq!(
        "SHR64 @R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::SHR), 0b10101001])
    );

    assert_eq!(
        "SUB32 R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::SUB), 0b00100001])
    );

    assert_eq!(
        "SUB32 @R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::SUB), 0b00101001])
    );

    assert_eq!(
        "SUB32 R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::SUB), 0b10100001])
    );

    assert_eq!(
        "SUB32 @R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::SUB), 0b10101001])
    );

    assert_eq!(
        "SUB64 R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::SUB), 0b00100001])
    );

    assert_eq!(
        "SUB64 @R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::SUB), 0b00101001])
    );

    assert_eq!(
        "SUB64 R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::SUB), 0b10100001])
    );

    assert_eq!(
        "SUB64 @R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::SUB), 0b10101001])
    );

    assert_eq!(
        "XOR32 R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::XOR), 0b00100001])
    );

    assert_eq!(
        "XOR32 @R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::XOR), 0b00101001])
    );

    assert_eq!(
        "XOR32 R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::XOR), 0b10100001])
    );

    assert_eq!(
        "XOR32 @R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::XOR), 0b10101001])
    );

    assert_eq!(
        "XOR64 R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::XOR), 0b00100001])
    );

    assert_eq!(
        "XOR64 @R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::XOR), 0b00101001])
    );

    assert_eq!(
        "XOR64 R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::XOR), 0b10100001])
    );

    assert_eq!(
        "XOR64 @R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::XOR), 0b10101001])
    );

    assert_eq!(
        "CMPeq32 R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::CMPeq), 0b00100001])
    );

    assert_eq!(
        "CMPeq32 @R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::CMPeq), 0b00101001])
    );

    assert_eq!(
        "CMPeq32 R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::CMPeq), 0b10100001])
    );

    assert_eq!(
        "CMPeq32 @R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::CMPeq), 0b10101001])
    );

    assert_eq!(
        "CMPeq64 R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::CMPeq), 0b00100001])
    );

    assert_eq!(
        "CMPeq64 @R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::CMPeq), 0b00101001])
    );

    assert_eq!(
        "CMPeq64 R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::CMPeq), 0b10100001])
    );

    assert_eq!(
        "CMPeq64 @R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::CMPeq), 0b10101001])
    );

    assert_eq!(
        "CMPlte32 R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::CMPlte), 0b00100001])
    );

    assert_eq!(
        "CMPlte32 @R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::CMPlte), 0b00101001])
    );

    assert_eq!(
        "CMPlte32 R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::CMPlte), 0b10100001])
    );

    assert_eq!(
        "CMPlte32 @R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::CMPlte), 0b10101001])
    );

    assert_eq!(
        "CMPlte64 R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::CMPlte), 0b00100001])
    );

    assert_eq!(
        "CMPlte64 @R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::CMPlte), 0b00101001])
    );

    assert_eq!(
        "CMPlte64 R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::CMPlte), 0b10100001])
    );

    assert_eq!(
        "CMPlte64 @R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::CMPlte), 0b10101001])
    );

    assert_eq!(
        "CMPgte32 R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::CMPgte), 0b00100001])
    );

    assert_eq!(
        "CMPgte32 @R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::CMPgte), 0b00101001])
    );

    assert_eq!(
        "CMPgte32 R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::CMPgte), 0b10100001])
    );

    assert_eq!(
        "CMPgte32 @R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::CMPgte), 0b10101001])
    );

    assert_eq!(
        "CMPgte64 R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::CMPgte), 0b00100001])
    );

    assert_eq!(
        "CMPgte64 @R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::CMPgte), 0b00101001])
    );

    assert_eq!(
        "CMPgte64 R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::CMPgte), 0b10100001])
    );

    assert_eq!(
        "CMPgte64 @R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::CMPgte), 0b10101001])
    );

    assert_eq!(
        "CMPulte32 R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::CMPulte), 0b00100001])
    );

    assert_eq!(
        "CMPulte32 @R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::CMPulte), 0b00101001])
    );

    assert_eq!(
        "CMPulte32 R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::CMPulte), 0b10100001])
    );

    assert_eq!(
        "CMPulte32 @R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::CMPulte), 0b10101001])
    );

    assert_eq!(
        "CMPulte64 R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::CMPulte), 0b00100001])
    );

    assert_eq!(
        "CMPulte64 @R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::CMPulte), 0b00101001])
    );

    assert_eq!(
        "CMPulte64 R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::CMPulte), 0b10100001])
    );

    assert_eq!(
        "CMPulte64 @R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::CMPulte), 0b10101001])
    );

    assert_eq!(
        "CMPugte32 R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::CMPugte), 0b00100001])
    );

    assert_eq!(
        "CMPugte32 @R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::CMPugte), 0b00101001])
    );

    assert_eq!(
        "CMPugte32 R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::CMPugte), 0b10100001])
    );

    assert_eq!(
        "CMPugte32 @R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::CMPugte), 0b10101001])
    );

    assert_eq!(
        "CMPugte64 R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::CMPugte), 0b00100001])
    );

    assert_eq!(
        "CMPugte64 @R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::CMPugte), 0b00101001])
    );

    assert_eq!(
        "CMPugte64 R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::CMPugte), 0b10100001])
    );

    assert_eq!(
        "CMPugte64 @R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::CMPugte), 0b10101001])
    );

    assert_eq!(
        "MUL32 R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::MUL), 0b00100001])
    );

    assert_eq!(
        "MUL32 @R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::MUL), 0b00101001])
    );

    assert_eq!(
        "MUL32 R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::MUL), 0b10100001])
    );

    assert_eq!(
        "MUL32 @R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::MUL), 0b10101001])
    );

    assert_eq!(
        "MUL64 R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::MUL), 0b00100001])
    );

    assert_eq!(
        "MUL64 @R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::MUL), 0b00101001])
    );

    assert_eq!(
        "MUL64 R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::MUL), 0b10100001])
    );

    assert_eq!(
        "MUL64 @R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::MUL), 0b10101001])
    );

    assert_eq!(
        "MULU32 R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::MULU), 0b00100001])
    );

    assert_eq!(
        "MULU32 @R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::MULU), 0b00101001])
    );

    assert_eq!(
        "MULU32 R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::MULU), 0b10100001])
    );

    assert_eq!(
        "MULU32 @R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::MULU), 0b10101001])
    );

    assert_eq!(
        "MULU64 R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::MULU), 0b00100001])
    );

    assert_eq!(
        "MULU64 @R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::MULU), 0b00101001])
    );

    assert_eq!(
        "MULU64 R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::MULU), 0b10100001])
    );

    assert_eq!(
        "MULU64 @R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::MULU), 0b10101001])
    );

    assert_eq!(
        "NEG32 R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::NEG), 0b00100001])
    );

    assert_eq!(
        "NEG32 @R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::NEG), 0b00101001])
    );

    assert_eq!(
        "NEG32 R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::NEG), 0b10100001])
    );

    assert_eq!(
        "NEG32 @R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::NEG), 0b10101001])
    );

    assert_eq!(
        "NEG64 R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::NEG), 0b00100001])
    );

    assert_eq!(
        "NEG64 @R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::NEG), 0b00101001])
    );

    assert_eq!(
        "NEG64 R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::NEG), 0b10100001])
    );

    assert_eq!(
        "NEG64 @R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::NEG), 0b10101001])
    );

    assert_eq!(
        "NOT32 R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::NOT), 0b00100001])
    );

    assert_eq!(
        "NOT32 @R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::NOT), 0b00101001])
    );

    assert_eq!(
        "NOT32 R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::NOT), 0b10100001])
    );

    assert_eq!(
        "NOT32 @R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::NOT), 0b10101001])
    );

    assert_eq!(
        "NOT64 R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::NOT), 0b00100001])
    );

    assert_eq!(
        "NOT64 @R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::NOT), 0b00101001])
    );

    assert_eq!(
        "NOT64 R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::NOT), 0b10100001])
    );

    assert_eq!(
        "NOT64 @R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::NOT), 0b10101001])
    );

    assert_eq!(
        "OR32 R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::OR), 0b00100001])
    );

    assert_eq!(
        "OR32 @R1, R2",
        dis(opts, cur, &[byte(0, 0, OpCode::OR), 0b00101001])
    );

    assert_eq!(
        "OR32 R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::OR), 0b10100001])
    );

    assert_eq!(
        "OR32 @R1, @R2",
        dis(opts, cur, &[byte(0, 0, OpCode::OR), 0b10101001])
    );

    assert_eq!(
        "OR64 R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::OR), 0b00100001])
    );

    assert_eq!(
        "OR64 @R1, R2",
        dis(opts, cur, &[byte(0, 1, OpCode::OR), 0b00101001])
    );

    assert_eq!(
        "OR64 R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::OR), 0b10100001])
    );

    assert_eq!(
        "OR64 @R1, @R2",
        dis(opts, cur, &[byte(0, 1, OpCode::OR), 0b10101001])
    );

    assert_eq!(
        "JMP32 R1 -3  ;; Absolute Address",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::JMP), 0b00000001][..],
                &(-3i32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "JMP32 R1 -3  ;; Relative Address",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::JMP), 0b00010001][..],
                &(-3i32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "JMP32 @R1(-300, -300)  ;; Absolute Address",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::JMP), 0b00001001][..],
                &(2954019116u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "JMP32 @R1(-300, -300)  ;; Relative Address",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::JMP), 0b00011001][..],
                &(2954019116u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "JMP64 1000  ;; Absolute Address",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::JMP), 0b00000001][..],
                &(1000u64).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "JMP64 1000  ;; Relative Address",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::JMP), 0b00010001][..],
                &(1000u64).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVIbw R1, 1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::MOVI), 0b00000001][..],
                &(1000u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVIww R1, 1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::MOVI), 0b00010001][..],
                &(1000u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVIdw R1, 1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::MOVI), 0b00100001][..],
                &(1000u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVIqw R1, 1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::MOVI), 0b00110001][..],
                &(1000u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVIbd R1, 1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MOVI), 0b00000001][..],
                &(1000u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVIwd R1, 1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MOVI), 0b00010001][..],
                &(1000u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVIdd R1, 1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MOVI), 0b00100001][..],
                &(1000u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVIqd R1, 1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MOVI), 0b00110001][..],
                &(1000u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVIbq R1, 1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MOVI), 0b00000001][..],
                &(1000u64).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVIwq R1, 1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MOVI), 0b00010001][..],
                &(1000u64).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVIdq R1, 1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MOVI), 0b00100001][..],
                &(1000u64).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVIqq R1, 1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MOVI), 0b00110001][..],
                &(1000u64).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI32weq R1, -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 0, OpCode::CMPIeq), 0b00000001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI64weq R1, -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::CMPIeq), 0b00000001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI32wlte R1, -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 0, OpCode::CMPIlte), 0b00000001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI64wlte R1, -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::CMPIlte), 0b00000001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI32wgte R1, -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 0, OpCode::CMPIgte), 0b00000001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI64wgte R1, -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::CMPIgte), 0b00000001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI32wulte R1, 1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 0, OpCode::CMPIulte), 0b00000001][..],
                &(1000u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI64wulte R1, 1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::CMPIulte), 0b00000001][..],
                &(1000u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI32wugte R1, 1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 0, OpCode::CMPIugte), 0b00000001][..],
                &(1000u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI64wugte R1, 1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::CMPIugte), 0b00000001][..],
                &(1000u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI32deq R1, -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CMPIeq), 0b00000001][..],
                &(-1000i32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI64deq R1, -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::CMPIeq), 0b00000001][..],
                &(-1000i32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI32dlte R1, -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CMPIlte), 0b00000001][..],
                &(-1000i32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI64dlte R1, -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::CMPIlte), 0b00000001][..],
                &(-1000i32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI32dgte R1, -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CMPIgte), 0b00000001][..],
                &(-1000i32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI64dgte R1, -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::CMPIgte), 0b00000001][..],
                &(-1000i32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI32dulte R1, 1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CMPIulte), 0b00000001][..],
                &(1000u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI64dulte R1, 1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::CMPIulte), 0b00000001][..],
                &(1000u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI32dugte R1, 1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CMPIugte), 0b00000001][..],
                &(1000u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI64dugte R1, 1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::CMPIugte), 0b00000001][..],
                &(1000u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI32weq @R1(-3, -3), -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 0, OpCode::CMPIeq), 0b00011001][..],
                &(36879u16).to_le_bytes()[..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI64weq @R1(-3, -3), -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::CMPIeq), 0b00011001][..],
                &(36879u16).to_le_bytes()[..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI32wlte @R1(-3, -3), -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 0, OpCode::CMPIlte), 0b00011001][..],
                &(36879u16).to_le_bytes()[..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI64wlte @R1(-3, -3), -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::CMPIlte), 0b00011001][..],
                &(36879u16).to_le_bytes()[..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI32wgte @R1(-3, -3), -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 0, OpCode::CMPIgte), 0b00011001][..],
                &(36879u16).to_le_bytes()[..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI64wgte @R1(-3, -3), -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::CMPIgte), 0b00011001][..],
                &(36879u16).to_le_bytes()[..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI32wulte @R1(-3, -3), 1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 0, OpCode::CMPIulte), 0b00011001][..],
                &(36879u16).to_le_bytes()[..],
                &(1000u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI64wulte @R1(-3, -3), 1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::CMPIulte), 0b00011001][..],
                &(36879u16).to_le_bytes()[..],
                &(1000u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI32wugte @R1(-3, -3), 1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 0, OpCode::CMPIugte), 0b00011001][..],
                &(36879u16).to_le_bytes()[..],
                &(1000u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI64wugte @R1(-3, -3), 1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::CMPIugte), 0b00011001][..],
                &(36879u16).to_le_bytes()[..],
                &(1000u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI32deq @R1(-3, -3), -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CMPIeq), 0b00011001][..],
                &(36879u16).to_le_bytes()[..],
                &(-1000i32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI64deq @R1(-3, -3), -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::CMPIeq), 0b00011001][..],
                &(36879u16).to_le_bytes()[..],
                &(-1000i32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI32dlte @R1(-3, -3), -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CMPIlte), 0b00011001][..],
                &(36879u16).to_le_bytes()[..],
                &(-1000i32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI64dlte @R1(-3, -3), -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::CMPIlte), 0b00011001][..],
                &(36879u16).to_le_bytes()[..],
                &(-1000i32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI32dgte @R1(-3, -3), -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CMPIgte), 0b00011001][..],
                &(36879u16).to_le_bytes()[..],
                &(-1000i32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI64dgte @R1(-3, -3), -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::CMPIgte), 0b00011001][..],
                &(36879u16).to_le_bytes()[..],
                &(-1000i32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI32dulte @R1(-3, -3), 1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CMPIulte), 0b00011001][..],
                &(36879u16).to_le_bytes()[..],
                &(1000u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI64dulte @R1(-3, -3), 1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::CMPIulte), 0b00011001][..],
                &(36879u16).to_le_bytes()[..],
                &(1000u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI32dugte @R1(-3, -3), 1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CMPIugte), 0b00011001][..],
                &(36879u16).to_le_bytes()[..],
                &(1000u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPI64dugte @R1(-3, -3), 1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::CMPIugte), 0b00011001][..],
                &(36879u16).to_le_bytes()[..],
                &(1000u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVInw R1, (-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::MOVIn), 0b00000001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVInd R1, (-300, -300)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MOVIn), 0b00000001][..],
                &(2954019116u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVInq R1, (-30000, -30000)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MOVIn), 0b00000001][..],
                &(11529215048034579760u64).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVInw @R1(-3, -3), (-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::MOVIn), 0b01001001][..],
                &(36879u16).to_le_bytes()[..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVInd @R1(-3, -3), (-300, -300)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MOVIn), 0b01001001][..],
                &(36879u16).to_le_bytes()[..],
                &(2954019116u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVInq @R1(-3, -3), (-30000, -30000)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MOVIn), 0b01001001][..],
                &(36879u16).to_le_bytes()[..],
                &(11529215048034579760u64).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVRELw R1, -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::MOVREL), 0b00000001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVRELd R1, -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MOVREL), 0b00000001][..],
                &(-1000i32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVRELq R1, -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MOVREL), 0b00000001][..],
                &(-1000i64).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVRELw @R1(-3, -3), -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::MOVREL), 0b01001001][..],
                &(36879u16).to_le_bytes()[..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVRELd @R1(-3, -3), -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MOVREL), 0b01001001][..],
                &(36879u16).to_le_bytes()[..],
                &(-1000i32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVRELq @R1(-3, -3), -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MOVREL), 0b01001001][..],
                &(36879u16).to_le_bytes()[..],
                &(-1000i64).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVnw R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::MOVnw), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVnd R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::MOVnd), 0b00100001][..],
                &(-1000i32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVnw @R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::MOVnw), 0b00101001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVnd @R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::MOVnd), 0b00101001][..],
                &(-1000i32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVnw @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::MOVnw), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVnd @R1, @R2(-300, -300)",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::MOVnd), 0b10101001][..],
                &(2954019116u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVnw @R1(-3, -3), R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MOVnw), 0b00101001][..],
                &(36879u16).to_le_bytes()[..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVnd @R1(-300, -300), R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MOVnd), 0b00101001][..],
                &(2954019116u32).to_le_bytes()[..],
                &(-1000i32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVnw @R1(-3, -3), @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MOVnw), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVnd @R1(-300, -300), @R2(-300, -300)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MOVnd), 0b10101001][..],
                &(2954019116u32).to_le_bytes()[..],
                &(2954019116u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVsnw R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::MOVsnw), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVsnd R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::MOVsnd), 0b00100001][..],
                &(-1000i32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVsnw @R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::MOVsnw), 0b00101001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVsnd @R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::MOVsnd), 0b00101001][..],
                &(-1000i32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVsnw @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::MOVsnw), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVsnd @R1, @R2(-300, -300)",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::MOVsnd), 0b10101001][..],
                &(2954019116u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVsnw @R1(-3, -3), R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MOVsnw), 0b00101001][..],
                &(36879u16).to_le_bytes()[..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVsnd @R1(-300, -300), R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MOVsnd), 0b00101001][..],
                &(2954019116u32).to_le_bytes()[..],
                &(-1000i32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVsnw @R1(-3, -3), @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MOVsnw), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVsnd @R1(-300, -300), @R2(-300, -300)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MOVsnd), 0b10101001][..],
                &(2954019116u32).to_le_bytes()[..],
                &(2954019116u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVbw @R1(-3, -3), R2",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MOVbw), 0b00101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVww @R1(-3, -3), R2",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MOVww), 0b00101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVdw @R1(-3, -3), R2",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MOVdw), 0b00101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVqw @R1(-3, -3), R2",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MOVqw), 0b00101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVbw @R1(-3, -3), @R2",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MOVbw), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVww @R1(-3, -3), @R2",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MOVww), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVdw @R1(-3, -3), @R2",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MOVdw), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVqw @R1(-3, -3), @R2",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MOVqw), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVbw R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::MOVbw), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVww R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::MOVww), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVdw R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::MOVdw), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVqw R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::MOVqw), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVbw @R1(-3, -3), @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MOVbw), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVww @R1(-3, -3), @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MOVww), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVdw @R1(-3, -3), @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MOVdw), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVqw @R1(-3, -3), @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MOVqw), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVbd @R1(-300, -300), R2",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MOVbd), 0b00101001][..],
                &(2954019116u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVwd @R1(-300, -300), R2",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MOVwd), 0b00101001][..],
                &(2954019116u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVdd @R1(-300, -300), R2",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MOVdd), 0b00101001][..],
                &(2954019116u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVqd @R1(-300, -300), R2",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MOVqd), 0b00101001][..],
                &(2954019116u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVbd @R1(-300, -300), @R2",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MOVbd), 0b10101001][..],
                &(2954019116u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVwd @R1(-300, -300), @R2",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MOVwd), 0b10101001][..],
                &(2954019116u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVdd @R1(-300, -300), @R2",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MOVdd), 0b10101001][..],
                &(2954019116u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVqd @R1(-300, -300), @R2",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MOVqd), 0b10101001][..],
                &(2954019116u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVbd R1, @R2(-300, -300)",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::MOVbd), 0b10100001][..],
                &(2954019116u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVwd R1, @R2(-300, -300)",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::MOVwd), 0b10100001][..],
                &(2954019116u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVdd R1, @R2(-300, -300)",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::MOVdd), 0b10100001][..],
                &(2954019116u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVqd R1, @R2(-300, -300)",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::MOVqd), 0b10100001][..],
                &(2954019116u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVbd @R1(-300, -300), @R2(-300, -300)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MOVbd), 0b10101001][..],
                &(2954019116u32).to_le_bytes()[..],
                &(2954019116u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVwd @R1(-300, -300), @R2(-300, -300)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MOVwd), 0b10101001][..],
                &(2954019116u32).to_le_bytes()[..],
                &(2954019116u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVdd @R1(-300, -300), @R2(-300, -300)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MOVdd), 0b10101001][..],
                &(2954019116u32).to_le_bytes()[..],
                &(2954019116u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVqd @R1(-300, -300), @R2(-300, -300)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MOVqd), 0b10101001][..],
                &(2954019116u32).to_le_bytes()[..],
                &(2954019116u32).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVqq @R1(-30000, -30000), R2",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MOVqq), 0b00101001][..],
                &(11529215048034579760u64).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVqq @R1(-30000, -30000), @R2",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MOVqq), 0b10101001][..],
                &(11529215048034579760u64).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVqq R1, @R2(-30000, -30000)",
        dis(
            opts,
            cur,
            &[
                &[byte(0, 1, OpCode::MOVqq), 0b10100001][..],
                &(11529215048034579760u64).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOVqq @R1(-30000, -30000), @R2(-30000, -30000)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MOVqq), 0b10101001][..],
                &(11529215048034579760u64).to_le_bytes()[..],
                &(11529215048034579760u64).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "ADD32 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::ADD), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "ADD32 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::ADD), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "ADD32 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::ADD), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "ADD64 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::ADD), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "ADD64 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::ADD), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "ADD64 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::ADD), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "AND32 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::AND), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "AND32 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::AND), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "AND32 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::AND), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "AND64 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::AND), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "AND64 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::AND), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "AND64 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::AND), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "ASHR32 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::ASHR), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "ASHR32 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::ASHR), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "ASHR32 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::ASHR), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "ASHR64 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::ASHR), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "ASHR64 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::ASHR), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "ASHR64 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::ASHR), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "DIV32 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::DIV), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "DIV32 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::DIV), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "DIV32 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::DIV), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "DIV64 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::DIV), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "DIV64 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::DIV), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "DIV64 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::DIV), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "DIVU32 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::DIVU), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "DIVU32 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::DIVU), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "DIVU32 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::DIVU), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "DIVU64 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::DIVU), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "DIVU64 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::DIVU), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "DIVU64 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::DIVU), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "EXTNDB32 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::EXTNDB), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "EXTNDB32 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::EXTNDB), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "EXTNDB32 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::EXTNDB), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "EXTNDB64 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::EXTNDB), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "EXTNDB64 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::EXTNDB), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "EXTNDB64 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::EXTNDB), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "EXTNDD32 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::EXTNDD), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "EXTNDD32 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::EXTNDD), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "EXTNDD32 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::EXTNDD), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "EXTNDD64 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::EXTNDD), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "EXTNDD64 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::EXTNDD), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "EXTNDD64 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::EXTNDD), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "EXTNDW32 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::EXTNDW), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "EXTNDW32 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::EXTNDW), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "EXTNDW32 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::EXTNDW), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "EXTNDW64 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::EXTNDW), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "EXTNDW64 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::EXTNDW), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "EXTNDW64 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::EXTNDW), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOD32 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MOD), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOD32 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MOD), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOD32 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MOD), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOD64 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MOD), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOD64 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MOD), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MOD64 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MOD), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MODU32 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MODU), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MODU32 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MODU), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MODU32 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MODU), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MODU64 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MODU), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MODU64 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MODU), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MODU64 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MODU), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "SHL32 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::SHL), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "SHL32 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::SHL), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "SHL32 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::SHL), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "SHL64 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::SHL), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "SHL64 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::SHL), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "SHL64 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::SHL), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "SHR32 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::SHR), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "SHR32 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::SHR), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "SHR32 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::SHR), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "SHR64 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::SHR), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "SHR64 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::SHR), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "SHR64 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::SHR), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "SUB32 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::SUB), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "SUB32 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::SUB), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "SUB32 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::SUB), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "SUB64 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::SUB), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "SUB64 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::SUB), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "SUB64 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::SUB), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "XOR32 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::XOR), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "XOR32 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::XOR), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "XOR32 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::XOR), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "XOR64 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::XOR), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "XOR64 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::XOR), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "XOR64 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::XOR), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPeq32 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CMPeq), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPeq32 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CMPeq), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPeq32 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CMPeq), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPeq64 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::CMPeq), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPeq64 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::CMPeq), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPeq64 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::CMPeq), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPlte32 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CMPlte), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPlte32 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CMPlte), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPlte32 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CMPlte), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPlte64 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::CMPlte), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPlte64 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::CMPlte), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPlte64 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::CMPlte), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPgte32 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CMPgte), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPgte32 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CMPgte), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPgte32 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CMPgte), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPgte64 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::CMPgte), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPgte64 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::CMPgte), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPgte64 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::CMPgte), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPulte32 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CMPulte), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPulte32 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CMPulte), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPulte32 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CMPulte), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPulte64 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::CMPulte), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPulte64 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::CMPulte), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPulte64 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::CMPulte), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPugte32 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CMPugte), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPugte32 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CMPugte), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPugte32 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::CMPugte), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPugte64 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::CMPugte), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPugte64 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::CMPugte), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "CMPugte64 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::CMPugte), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MUL32 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MUL), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MUL32 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MUL), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MUL32 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MUL), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MUL64 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MUL), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MUL64 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MUL), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MUL64 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MUL), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MULU32 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MULU), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MULU32 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MULU), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MULU32 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::MULU), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MULU64 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MULU), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MULU64 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MULU), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "MULU64 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::MULU), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "NEG32 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::NEG), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "NEG32 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::NEG), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "NEG32 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::NEG), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "NEG64 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::NEG), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "NEG64 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::NEG), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "NEG64 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::NEG), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "NOT32 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::NOT), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "NOT32 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::NOT), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "NOT32 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::NOT), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "NOT64 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::NOT), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "NOT64 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::NOT), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "NOT64 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::NOT), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "OR32 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::OR), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "OR32 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::OR), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "OR32 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 0, OpCode::OR), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "OR64 R1, R2 -1000",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::OR), 0b00100001][..],
                &(-1000i16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "OR64 R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::OR), 0b10100001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );

    assert_eq!(
        "OR64 @R1, @R2(-3, -3)",
        dis(
            opts,
            cur,
            &[
                &[byte(1, 1, OpCode::OR), 0b10101001][..],
                &(36879u16).to_le_bytes()[..],
            ].concat()
        )
    );
}
