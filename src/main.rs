/*
Instruction Type Breakdown:

7. INSTRUCTION OP1 ARGUMENT, OP2 ARGUMENT
    MOVnw
    MOVnd
    MOVbw
    MOVww
    MOVdw
    MOVqw
    MOVbd
    MOVwd
    MOVdd
    MOVqd
    MOVqq
    MOVsnw
    MOVsnd

6. ✅ INSTRUCTION OP1, OP2 ARGUMENT (16 bit optional index/immediate)
    ADD
    AND
    ASHR
    CMP
    DIV
    DIVU
    EXTENDB
    EXTENDD
    EXTENDW
    MOD
    MODU
    MUL
    MULU
    NEG
    NOT
    OR
    SHL
    SHR
    SUB
    XOR

5. ✅ INSTRUCTION OP1 ARGUMENT, ARGUMENT
    CMPI
    MOVI
    MOVIn
    MOVREL

4. ✅ INSTRUCTION OP1, OP2
    STORESP
    LOADSP

3. ✅ INSTRUCTION OP1 ARGUMENT
    CALL32
    JMP32
    PUSH
    PUSHn
    POP
    POPn
    JMP64
    CALL64

2. ✅ INSTRUCTION ARGUMENT
    JMP8
    BREAK
    JMP64
    CALL64

1. ✅ INSTRUCTION
    RET
*/

// TODO(pbz): Perhaps put all the "Behaviors and Restrictions" bullet points in
// TODO(pbz): comments by each instruction so that you can read exact behavior.

// TODO(pbz): Replace panics/unreachable code with better error messages.
// TODO(pbz): I'm not sure if natural indexes need to be read using LE. BE?

use std::io::prelude::*;
use std::convert::TryInto;
use colored::*;

const BLUE: (u8, u8, u8) = (98, 168, 209);

enum Operand
{
    GeneralPurpose
    {
        register_index: u8,
        indirect: bool,
    },

    Dedicated
    {
        register_index: u8,
        indirect: bool,
    },
}

impl Operand
{
    pub fn new_general_purpose(register_index: u8, indirect: bool) -> Self
    {
        assert!((0u8 ..= 7u8).contains(&register_index));

        Self::GeneralPurpose
        {
            register_index,
            indirect
        }
    }

    pub fn new_dedicated(register_index: u8, indirect: bool) -> Self
    {
        assert!((0u8 ..= 1u8).contains(&register_index));

        Self::Dedicated
        {
            register_index,
            indirect
        }
    }
}

impl std::fmt::Display for Operand
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self
        {
            Self::GeneralPurpose { register_index: index, indirect: at } =>
            {
                assert!((0u8 ..= 7u8).contains(&index));

                // TODO(pbz): Is it inconsistent to not have a space here?
                // TODO(pbz): Other natural index arguments have a space.

                write!(
                    f,
                    "{}R{}",
                    if *at { "@" } else { "" },
                    index
                )
            }

            Self::Dedicated { register_index: index, indirect: at } =>
            {
                assert!((0u8 ..= 1u8).contains(&index));

                if *index == 0
                {
                    write!(f, "{}FLAGS", if *at { "@" } else { "" })
                }

                else
                {
                    write!(f, "{}IP", if *at { "@" } else { "" })
                }
            }
        }
    }
}

// ? Should this be genric over T for i32/u32 etc.
pub enum Argument
{
    Index16(u16),
    Index32(u32),
    Index64(u64),
    ImmediateU16(u16),
    ImmediateU32(u32),
    ImmediateU64(u64),
    ImmediateI16(i16),
    ImmediateI32(i32),
    ImmediateI64(i64),
}

impl std::fmt::Display for Argument
{
    // TODO(pbz): May want to remove + from here and only use - because it
    // TODO(pbz): doesn't make sense for BREAK CODE
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self
        {
            Self::Index16(index) =>
            {
                let natural_index = NaturalIndex::from_u16(*index);

                write!(f, "{}", natural_index)
            }

            Self::Index32(index) =>
            {
                let natural_index = NaturalIndex::from_u32(*index);

                write!(f, "{}", natural_index)
            }

            Self::Index64(index) =>
            {
                let natural_index = NaturalIndex::from_u64(*index);

                write!(f, "{}", natural_index)
            }

            Self::ImmediateU16(immediate) => write!(f, "{}", immediate),

            Self::ImmediateU32(immediate) => write!(f, "{}", immediate),

            Self::ImmediateU64(immediate) => write!(f, "{}", immediate),

            Self::ImmediateI16(immediate) => write!(f, "{}", immediate),

            Self::ImmediateI32(immediate) => write!(f, "{}", immediate),

            Self::ImmediateI64(immediate) => write!(f, "{}", immediate),
        }
    }
}

fn parse_instruction1<T: Iterator<Item=u8>>(
    _bytes: &mut T,
    _byte0_bits: [bool; 8],
    op: OpCode,
) -> Option<()>
{
    disassemble_instruction(
        format!("{}", op).truecolor(BLUE.0, BLUE.1, BLUE.2).to_string(),
        None,
        None,
        None,
        None,
        None
    );

    Some(())
}

fn parse_instruction2<T: Iterator<Item=u8>>(
    bytes: &mut T,
    byte0_bits: [bool; 8],
    op: OpCode,
) -> Option<()>
{
    let mut name = format!("{}", op);

    let byte1 = bytes.next().expect("Unexpected end of bytes");

    let arg1 = match op
    {
        OpCode::BREAK =>
        {
            if byte1 == 0
            {
                panic!(
                    "Runaway program break (found 2 zeros in a row, BREAK 0)"
                );
            }

            Argument::ImmediateU16(byte1 as u16)
        }

        OpCode::JMP8 =>
        {
            let conditional = byte0_bits[7];

            if conditional
            {
                let condition_bit_set = byte0_bits[6];

                name += if condition_bit_set
                {
                    "cs"
                }
                else
                {
                    "cc"
                };
            }

            Argument::ImmediateI16((byte1 as i8) as i16)
        }

        _ => unreachable!(),
    };

    disassemble_instruction(
        name.truecolor(BLUE.0, BLUE.1, BLUE.2).to_string(),
        None,
        Some(arg1),
        None,
        None,
        None
    );

    Some(())
}

fn parse_instruction3<T: Iterator<Item=u8>>(
    bytes: &mut T,
    byte0_bits: [bool; 8],
    op: OpCode,
) -> Option<()>
{
    let mut name = format!("{}", op);
    let mut postfix = String::with_capacity(5);
    let immediate_data_present = byte0_bits[7];
    let is_64_bit = byte0_bits[6];  // Not used by PUSHn & POPn

    let byte1 = bytes.next().expect("Unexpected end of bytes");
    let byte1_bits = bits_rev(byte1);

    match op
    {
        OpCode::CALL
        | OpCode::JMP
        | OpCode::PUSH
        | OpCode::POP =>
        {
            postfix += if is_64_bit { "64" } else { "32" };
        }

        _ => (),
    }

    // TODO(pbz): Have postfixes colored differently? =)
    let (op1, arg1, op2, arg2, comment) = match op
    {
        // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        // TODO(pbz): THIS IS VERY IMPORTANT. CHECK THIS VERY CAREFULLY
        // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        OpCode::CALL =>
        {
            let is_native_call = byte1_bits[5];
            postfix += if is_native_call { "EX" } else { "" };

            let is_relative_address = byte1_bits[4];
            let operand1_is_indirect = byte1_bits[3];
            let operand1_value = bits_to_byte_rev(&byte1_bits[0 ..= 2]);
            let op1 = if !is_64_bit
            {
                Some(
                    Operand::new_general_purpose(
                        operand1_value,
                        operand1_is_indirect
                    )
                )
            }
            else
            {
                None
            };

            let arg1 = if is_64_bit
            {
                postfix += "a";  // CALL64 is always an absolute address

                let mut value = [0u8; 8];

                for i in 0 .. value.len()
                {
                    value[i] = bytes.next().unwrap();
                }

                // TODO(pbz): For absolute, display in hex
                Some(Argument::ImmediateI64(i64::from_le_bytes(value)))
            }
            else
            {
                postfix += if is_relative_address { "" } else { "a" };

                let arg = if immediate_data_present
                {
                    let mut value = [0u8; 4];

                    for i in 0 .. value.len()
                    {
                        value[i] = bytes.next().unwrap();
                    }

                    if operand1_is_indirect
                    {
                        Some(Argument::Index32(u32::from_le_bytes(value)))
                    }
                    else
                    {
                        Some(Argument::ImmediateI32(i32::from_le_bytes(value)))
                    }
                }
                else
                {
                    None
                };

                arg
            };

            (op1, arg1, None, None, None)
        }

        OpCode::JMP =>
        {
            let conditional_jump = byte1_bits[7];
            let jump_if_condition_bit_set = byte1_bits[6];

            if conditional_jump
            {
                postfix += if jump_if_condition_bit_set { "cs" } else { "cc" };
            }

            let relative_address = byte1_bits[4];
            let operand1_is_indirect = byte1_bits[3];
            let operand1_value = bits_to_byte_rev(&byte1_bits[0 ..= 2]);
            let op1 = if !is_64_bit
            {
                Some(
                    Operand::new_general_purpose(
                        operand1_value,
                        operand1_is_indirect
                    )
                )
            }
            else
            {
                None
            };

            let arg1 = if is_64_bit
            {
                let mut value = [0u8; 8];

                for i in 0 .. value.len()
                {
                    value[i] = bytes.next().unwrap();
                }

                // TODO(pbz): Check if absolute, then display in hex
                Some(Argument::ImmediateI64(i64::from_le_bytes(value)))
            }
            else
            {
                let arg = if immediate_data_present
                {
                    let mut value = [0u8; 4];

                    for i in 0 .. value.len()
                    {
                        value[i] = bytes.next().unwrap();
                    }

                    if operand1_is_indirect
                    {
                        Some(Argument::Index32(u32::from_le_bytes(value)))
                    }
                    else
                    {
                        Some(Argument::ImmediateI32(i32::from_le_bytes(value)))
                    }
                }
                else
                {
                    None
                };

                arg
            };

            let comment = if relative_address
            {
                Some(String::from("Relative Address"))
            }
            else
            {
                Some(String::from("Absolute Address"))
            };

            (op1, arg1, None, None, comment)
        }

        OpCode::PUSH
        | OpCode::POP
        | OpCode::PUSHn
        | OpCode::POPn =>
        {
            let operand1_is_indirect = byte1_bits[3];
            let operand1_value = bits_to_byte_rev(&byte1_bits[0 ..= 2]);
            let arg1 = if immediate_data_present
            {
                let mut value = [0u8; 2];

                value[0] = bytes.next().unwrap();
                value[1] = bytes.next().unwrap();

                let arg = if operand1_is_indirect
                {
                    Argument::Index16(u16::from_le_bytes(value))
                }
                else
                {
                    Argument::ImmediateI16(i16::from_le_bytes(value))
                };

                Some(arg)
            }
            else
            {
                None
            };

            (
                Some(
                    Operand::new_general_purpose(
                        operand1_value,
                        operand1_is_indirect
                    )
                ),
                arg1,
                None,
                None,
                None
            )
        }

        _ => unreachable!(),
    };

    name += &postfix;

    disassemble_instruction(
        format!("{}", name).truecolor(BLUE.0, BLUE.1, BLUE.2).to_string(),
        op1,
        arg1,
        op2,
        arg2,
        comment
    );

    Some(())
}

fn parse_instruction4<T: Iterator<Item=u8>>(
    bytes: &mut T,
    _byte0_bits: [bool; 8],
    op: OpCode,
) -> Option<()>
{
    let name = format!("{}", op);

    let byte1 = bytes.next().expect("Unexpected end of bytes");
    let byte1_bits = bits_rev(byte1);
    let operand1_value = bits_to_byte_rev(&byte1_bits[0 ..= 2]);
    let operand2_value = bits_to_byte_rev(&byte1_bits[4 ..= 6]);

    let (op1, op2) = match op
    {
        OpCode::STORESP => (
            Operand::new_general_purpose(operand1_value, false),
            Operand::new_dedicated(operand2_value, false)
        ),

        OpCode::LOADSP => (
            Operand::new_dedicated(operand1_value, false),
            Operand::new_general_purpose(operand2_value, false)
        ),

        _ => unreachable!(),
    };

    disassemble_instruction(
        name.truecolor(BLUE.0, BLUE.1, BLUE.2).to_string(),
        Some(op1),
        None,
        Some(op2),
        None,
        None
    );

    Some(())
}

fn parse_instruction5<T: Iterator<Item=u8>>(
    bytes: &mut T,
    byte0_bits: [bool; 8],
    op: OpCode,
) -> Option<()>
{
    let mut name = format!("{}", op);
    // TODO(pbz): Have postfixes colored differently? =)
    let mut postfixes = String::with_capacity(7);

    let (op1, arg1, arg2) = match op
    {
        OpCode::MOVI =>
        {
            let byte1 = bytes.next().expect("Unexpected end of bytes");
            let byte1_bits = bits_rev(byte1);

            let move_width = bits_to_byte_rev(&byte1_bits[4 ..= 5]);
            postfixes += match move_width
            {
                0 => "b",  // 8 bit
                1 => "w",  // 16 bit
                2 => "d",  // 32 bit
                3 => "q",  // 64 bit
                _ => unreachable!(),
            };

            let immediate_data_width = bits_to_byte_rev(&byte0_bits[6 ..= 7]);
            postfixes += match immediate_data_width
            {
                1 => "w",  // 16 bit
                2 => "d",  // 32 bit
                3 => "q",  // 64 bit
                _ => unreachable!(),
            };

            let operand1_index_present = byte1_bits[6];
            let operand1_is_indirect = byte1_bits[3];
            let operand1_value = bits_to_byte_rev(&byte1_bits[0 ..= 2]);

            let op1 = Some(
                Operand::new_general_purpose(
                    operand1_value,
                    operand1_is_indirect
                )
            );

            let arg1 = if operand1_index_present
            {
                let mut value = [0u8; 2];

                value[0] = bytes.next().unwrap();
                value[1] = bytes.next().unwrap();

                let arg = if operand1_is_indirect
                {
                    Argument::Index16(u16::from_le_bytes(value))
                }
                else
                {
                    panic!("Immediate data not supported for CMPI");
                };

                Some(arg)
            }
            else
            {
                None
            };

            let arg2 = {
                match immediate_data_width
                {
                    1 =>  // 16 bit
                    {
                        let mut value = [0u8; 2];

                        for i in 0 .. value.len()
                        {
                            value[i] = bytes.next().unwrap();
                        }

                        Some(Argument::ImmediateI16(i16::from_le_bytes(value)))
                    }

                    2 =>  // 32 bit
                    {
                        let mut value = [0u8; 4];

                        for i in 0 .. value.len()
                        {
                            value[i] = bytes.next().unwrap();
                        }

                        Some(Argument::ImmediateI32(i32::from_le_bytes(value)))
                    }

                    3 =>  // 64 bit
                    {
                        let mut value = [0u8; 8];

                        for i in 0 .. value.len()
                        {
                            value[i] = bytes.next().unwrap();
                        }

                        Some(Argument::ImmediateI64(i64::from_le_bytes(value)))
                    }

                    _ => unreachable!(),
                }
            };

            name += &postfixes;

            (op1, arg1, arg2)
        }

        OpCode::CMPIeq
        | OpCode::CMPIlte
        | OpCode::CMPIgte
        | OpCode::CMPIulte
        | OpCode::CMPIugte =>
        {
            let immediate_data_is_32_bit = byte0_bits[7];
            let comparison_is_64_bit = byte0_bits[6];

            // Have to obliterate name due to the reordering below:
            name = String::from("CMPI");
            name += if comparison_is_64_bit { "64" } else { "32" };
            name += if immediate_data_is_32_bit { "d" } else { "w" };
            name += match op
            {
                OpCode::CMPIeq => "eq",
                OpCode::CMPIlte => "lte",
                OpCode::CMPIgte => "gte",
                OpCode::CMPIulte => "ulte",
                OpCode::CMPIugte => "ugte",
                _ => unreachable!(),
            };

            let byte1 = bytes.next().expect("Unexpected end of bytes");
            let byte1_bits = bits_rev(byte1);
            let operand1_index_present = byte1_bits[4];
            let operand1_is_indirect = byte1_bits[3];
            let operand1_value = bits_to_byte_rev(&byte1_bits[0 ..= 2]);

            let op1 = Some(
                Operand::new_general_purpose(
                    operand1_value,
                    operand1_is_indirect
                )
            );

            // TODO(pbz): Make this into a function for all to use
            let arg1 = if operand1_index_present
            {
                let mut value = [0u8; 2];

                value[0] = bytes.next().unwrap();
                value[1] = bytes.next().unwrap();

                let arg = if operand1_is_indirect
                {
                    Argument::Index16(u16::from_le_bytes(value))
                }
                else
                {
                    panic!("Immediate data not supported for CMPI");
                };

                Some(arg)
            }
            else
            {
                None
            };

            let arg2 = {
                if immediate_data_is_32_bit
                {
                    let mut value = [0u8; 4];

                    for i in 0 .. value.len()
                    {
                        value[i] = bytes.next().unwrap();
                    }

                    match op
                    {
                        OpCode::CMPIulte | OpCode::CMPIugte => Some(
                            Argument::ImmediateU32(u32::from_le_bytes(value))
                        ),

                        _ => Some(
                            Argument::ImmediateI32(i32::from_le_bytes(value))
                        ),
                    }
                }
                else
                {
                    let mut value = [0u8; 2];

                    for i in 0 .. value.len()
                    {
                        value[i] = bytes.next().unwrap();
                    }

                    match op
                    {
                        OpCode::CMPIulte | OpCode::CMPIugte => Some(
                            Argument::ImmediateU16(u16::from_le_bytes(value))
                        ),

                        _ => Some(
                            Argument::ImmediateI16(i16::from_le_bytes(value))
                        ),
                    }
                }
            };

            (op1, arg1, arg2)
        }

        OpCode::MOVIn =>
        {
            let operand2_index_width = bits_to_byte_rev(&byte0_bits[6 ..= 7]);
            postfixes += match operand2_index_width
            {
                1 => "w",  // 16 bit
                2 => "d",  // 32 bit
                3 => "q",  // 64 bit
                _ => unreachable!(),
            };

            let byte1 = bytes.next().expect("Unexpected end of bytes");
            let byte1_bits = bits_rev(byte1);
            let operand1_index_present = byte1_bits[6];
            let operand1_is_indirect = byte1_bits[3];
            let operand1_value = bits_to_byte_rev(&byte1_bits[0 ..= 2]);

            let op1 = Some(
                Operand::new_general_purpose(
                    operand1_value,
                    operand1_is_indirect
                )
            );

            let arg1 = if operand1_index_present
            {
                let mut value = [0u8; 2];

                value[0] = bytes.next().unwrap();
                value[1] = bytes.next().unwrap();

                let arg = if operand1_is_indirect
                {
                    Argument::Index16(u16::from_le_bytes(value))
                }
                else
                {
                    panic!("Immediate data not supported for CMPI");
                };

                Some(arg)
            }
            else
            {
                None
            };

            let arg2 = {
                match operand2_index_width
                {
                    1 =>  // 16 bit
                    {
                        let mut value = [0u8; 2];

                        for i in 0 .. value.len()
                        {
                            value[i] = bytes.next().unwrap();
                        }

                        Some(Argument::Index16(u16::from_le_bytes(value)))
                    }

                    2 =>  // 32 bit
                    {
                        let mut value = [0u8; 4];

                        for i in 0 .. value.len()
                        {
                            value[i] = bytes.next().unwrap();
                        }

                        Some(Argument::Index32(u32::from_le_bytes(value)))
                    }

                    3 =>  // 64 bit
                    {
                        let mut value = [0u8; 8];

                        for i in 0 .. value.len()
                        {
                            value[i] = bytes.next().unwrap();
                        }

                        Some(Argument::Index64(u64::from_le_bytes(value)))
                    }

                    _ => unreachable!(),
                }
            };

            name += &postfixes;

            (op1, arg1, arg2)
        }

        OpCode::MOVREL =>
        {
            let immediate_data_width = bits_to_byte_rev(&byte0_bits[6 ..= 7]);
            postfixes += match immediate_data_width
            {
                1 => "w",  // 16 bit
                2 => "d",  // 32 bit
                3 => "q",  // 64 bit
                _ => unreachable!(),
            };

            let byte1 = bytes.next().expect("Unexpected end of bytes");
            let byte1_bits = bits_rev(byte1);
            let operand1_index_present = byte1_bits[6];
            let operand1_is_indirect = byte1_bits[3];
            let operand1_value = bits_to_byte_rev(&byte1_bits[0 ..= 2]);

            let op1 = Some(
                Operand::new_general_purpose(
                    operand1_value,
                    operand1_is_indirect
                )
            );

            let arg1 = if operand1_index_present
            {
                let mut value = [0u8; 2];

                value[0] = bytes.next().unwrap();
                value[1] = bytes.next().unwrap();

                let arg = if operand1_is_indirect
                {
                    Argument::Index16(u16::from_le_bytes(value))
                }
                else
                {
                    panic!("Immediate data not supported for CMPI");
                };

                Some(arg)
            }
            else
            {
                None
            };

            let arg2 = {
                match immediate_data_width
                {
                    1 =>  // 16 bit
                    {
                        let mut value = [0u8; 2];

                        for i in 0 .. value.len()
                        {
                            value[i] = bytes.next().unwrap();
                        }

                        Some(Argument::ImmediateI16(i16::from_le_bytes(value)))
                    }

                    2 =>  // 32 bit
                    {
                        let mut value = [0u8; 4];

                        for i in 0 .. value.len()
                        {
                            value[i] = bytes.next().unwrap();
                        }

                        Some(Argument::ImmediateI32(i32::from_le_bytes(value)))
                    }

                    3 =>  // 64 bit
                    {
                        let mut value = [0u8; 8];

                        for i in 0 .. value.len()
                        {
                            value[i] = bytes.next().unwrap();
                        }

                        Some(Argument::ImmediateI64(i64::from_le_bytes(value)))
                    }

                    _ => unreachable!(),
                }
            };

            name += &postfixes;

            (op1, arg1, arg2)
        }

        _ => unreachable!(),
    };

    disassemble_instruction(
        name.truecolor(BLUE.0, BLUE.1, BLUE.2).to_string(),
        op1,
        arg1,
        None,
        arg2,
        None
    );

    Some(())
}

fn parse_instruction6<T: Iterator<Item=u8>>(
    bytes: &mut T,
    byte0_bits: [bool; 8],
    op: OpCode,
) -> Option<()>
{
    let mut name = format!("{}", op);
    let immediate_data_present = byte0_bits[7];

    // TODO(pbz): Have postfixes colored differently? =)
    name += if byte0_bits[6]
    {
        "64"
    }
    else
    {
        "32"
    };

    let byte1 = bytes.next().expect("Unexpected end of bytes");
    let byte1_bits = bits_rev(byte1);
    let operand1_is_indirect = byte1_bits[3];
    let operand1_value = bits_to_byte_rev(&byte1_bits[0 ..= 2]);
    let operand2_is_indirect = byte1_bits[7];
    let operand2_value = bits_to_byte_rev(&byte1_bits[4 ..= 6]);

    let op1_x16_index_or_immediate =
    {
        if immediate_data_present
        {
            let mut value = [0u8; 2];

            value[0] = bytes.next().unwrap();
            value[1] = bytes.next().unwrap();

            let arg = if operand2_is_indirect
            {
                Argument::Index16(u16::from_le_bytes(value))
            }
            else
            {
                Argument::ImmediateI16(i16::from_le_bytes(value))
            };

            Some(arg)
        }
        else
        {
            None
        }
    };

    disassemble_instruction(
        name.truecolor(BLUE.0, BLUE.1, BLUE.2).to_string(),
        Some(
            Operand::new_general_purpose(operand1_value, operand1_is_indirect)
        ),
        None,
        Some(
            Operand::new_general_purpose(operand2_value, operand2_is_indirect)
        ),
        op1_x16_index_or_immediate,
        None
    );

    Some(())
}

fn parse_instruction7<T: Iterator<Item=u8>>(
    bytes: &mut T,
    byte0_bits: [bool; 8],
    op: OpCode,
) -> Option<()>
{
    let mut name = format!("{}", op);
    let operand1_index_present = byte0_bits[7];
    let operand2_index_present = byte0_bits[6];

    // TODO(pbz): Really need to color-code the bit widths (bwdq)

    // Can't presume operands are indexed
    if !(operand1_index_present || operand2_index_present)
    {
        name = name[.. name.len() - 1].to_string();
    }

    let byte1 = bytes.next().expect("Unexpected end of bytes");
    let byte1_bits = bits_rev(byte1);
    let operand1_is_indirect = byte1_bits[3];
    let operand1_value = bits_to_byte_rev(&byte1_bits[0 ..= 2]);
    let operand2_is_indirect = byte1_bits[7];
    let operand2_value = bits_to_byte_rev(&byte1_bits[4 ..= 6]);

    let op1 = Some(
        Operand::new_general_purpose(
            operand1_value,
            operand1_is_indirect
        )
    );

    let op2 = Some(
        Operand::new_general_purpose(
            operand2_value,
            operand2_is_indirect
        )
    );

    let (arg1, arg2) = match op
    {
        OpCode::MOVsnw
        | OpCode::MOVsnd =>
        {
            let arg1 =
            {
                if operand1_index_present
                {
                    let arg = match op
                    {
                        OpCode::MOVsnw =>  // 16 bit
                        {
                            let mut value = [0u8; 2];

                            for i in 0 .. value.len()
                            {
                                value[i] = bytes.next().unwrap();
                            }

                            Argument::Index16(u16::from_le_bytes(value))
                        }

                        OpCode::MOVsnd =>  // 32 bit
                        {
                            let mut value = [0u8; 4];

                            for i in 0 .. value.len()
                            {
                                value[i] = bytes.next().unwrap();
                            }

                            Argument::Index32(u32::from_le_bytes(value))
                        }

                        _ => unreachable!()
                    };

                    Some(arg)
                }
                else
                {
                    None
                }
            };

            let arg2 =
            {
                if operand2_index_present
                {
                    let arg = match op
                    {
                        OpCode::MOVsnw =>  // 16 bit
                        {
                            let mut value = [0u8; 2];

                            for i in 0 .. value.len()
                            {
                                value[i] = bytes.next().unwrap();
                            }

                            if operand2_is_indirect
                            {
                                Argument::Index16(u16::from_le_bytes(value))
                            }
                            else
                            {
                                Argument::ImmediateI16(
                                    i16::from_le_bytes(value)
                                )
                            }
                        }

                        OpCode::MOVsnd =>  // 32 bit
                        {
                            let mut value = [0u8; 4];

                            for i in 0 .. value.len()
                            {
                                value[i] = bytes.next().unwrap();
                            }

                            if operand2_is_indirect
                            {
                                Argument::Index32(u32::from_le_bytes(value))
                            }
                            else
                            {
                                Argument::ImmediateI32(
                                    i32::from_le_bytes(value)
                                )
                            }
                        }

                        _ => unreachable!()
                    };

                    Some(arg)
                }
                else
                {
                    None
                }
            };

            (arg1, arg2)
        }

        // Try to combine this with MOVsnw and MOVsnd
        OpCode::MOVnw
        | OpCode::MOVnd =>
        {
            let arg1 =
            {
                if operand1_index_present
                {
                    let arg = match op
                    {
                        OpCode::MOVnw =>  // 16 bit
                        {
                            let mut value = [0u8; 2];

                            for i in 0 .. value.len()
                            {
                                value[i] = bytes.next().unwrap();
                            }

                            Argument::Index16(u16::from_le_bytes(value))
                        }

                        OpCode::MOVnd =>  // 32 bit
                        {
                            let mut value = [0u8; 4];

                            for i in 0 .. value.len()
                            {
                                value[i] = bytes.next().unwrap();
                            }

                            Argument::Index32(u32::from_le_bytes(value))
                        }

                        _ => unreachable!()
                    };

                    Some(arg)
                }
                else
                {
                    None
                }
            };

            let arg2 =
            {
                if operand2_index_present
                {
                    let arg = match op
                    {
                        OpCode::MOVnw =>  // 16 bit
                        {
                            let mut value = [0u8; 2];

                            for i in 0 .. value.len()
                            {
                                value[i] = bytes.next().unwrap();
                            }

                            if operand2_is_indirect
                            {
                                Argument::Index16(u16::from_le_bytes(value))
                            }
                            else
                            {
                                Argument::ImmediateI16(
                                    i16::from_le_bytes(value)
                                )
                            }
                        }

                        OpCode::MOVnd =>  // 32 bit
                        {
                            let mut value = [0u8; 4];

                            for i in 0 .. value.len()
                            {
                                value[i] = bytes.next().unwrap();
                            }

                            if operand2_is_indirect
                            {
                                Argument::Index32(u32::from_le_bytes(value))
                            }
                            else
                            {
                                Argument::ImmediateI32(
                                    i32::from_le_bytes(value)
                                )
                            }
                        }

                        _ => unreachable!()
                    };

                    Some(arg)
                }
                else
                {
                    None
                }
            };

            (arg1, arg2)
        }

        _ =>  // MOV
        {
            let arg1 =
            {
                if operand1_index_present
                {
                    let arg = match op
                    {
                        OpCode::MOVbw
                        | OpCode::MOVww
                        | OpCode::MOVdw
                        | OpCode::MOVqw =>  // 16 bit
                        {
                            let mut value = [0u8; 2];

                            for i in 0 .. value.len()
                            {
                                value[i] = bytes.next().unwrap();
                            }

                            Argument::Index16(u16::from_le_bytes(value))
                        }

                        OpCode::MOVbd
                        | OpCode::MOVwd
                        | OpCode::MOVdd
                        | OpCode::MOVqd =>  // 32 bit
                        {
                            let mut value = [0u8; 4];

                            for i in 0 .. value.len()
                            {
                                value[i] = bytes.next().unwrap();
                            }

                            Argument::Index32(u32::from_le_bytes(value))
                        }

                        OpCode::MOVqq =>  //64 bit
                        {
                            let mut value = [0u8; 8];

                            for i in 0 .. value.len()
                            {
                                value[i] = bytes.next().unwrap();
                            }

                            Argument::Index64(u64::from_le_bytes(value))
                        }

                        _ => unreachable!()
                    };

                    Some(arg)
                }
                else
                {
                    None
                }
            };

            let arg2 =
            {
                if operand2_index_present
                {
                    let arg = match op
                    {
                        OpCode::MOVbw
                        | OpCode::MOVww
                        | OpCode::MOVdw
                        | OpCode::MOVqw =>  // 16 bit
                        {
                            let mut value = [0u8; 2];

                            for i in 0 .. value.len()
                            {
                                value[i] = bytes.next().unwrap();
                            }

                            Argument::Index16(u16::from_le_bytes(value))
                        }

                        OpCode::MOVbd
                        | OpCode::MOVwd
                        | OpCode::MOVdd
                        | OpCode::MOVqd =>  // 32 bit
                        {
                            let mut value = [0u8; 4];

                            for i in 0 .. value.len()
                            {
                                value[i] = bytes.next().unwrap();
                            }

                            Argument::Index32(u32::from_le_bytes(value))
                        }

                        OpCode::MOVqq =>  //64 bit
                        {
                            let mut value = [0u8; 8];

                            for i in 0 .. value.len()
                            {
                                value[i] = bytes.next().unwrap();
                            }

                            Argument::Index64(u64::from_le_bytes(value))
                        }

                        _ => unreachable!()
                    };

                    Some(arg)
                }
                else
                {
                    None
                }
            };

            (arg1, arg2)
        }
    };

    disassemble_instruction(
        name.truecolor(BLUE.0, BLUE.1, BLUE.2).to_string(),
        op1,
        arg1,
        op2,
        arg2,
        None
    );

    Some(())
}

// TODO(pbz): Output the actual bytecode bytes (machine code) side by side
// TODO(pbz): so that you can count the bytes for relative jumps!
// TODO(pbz): Only output text once. Build in buffer
// TODO(pbz): Invest in some left/right justification
// TODO(pbz): Justify in columns maybe?
fn disassemble_instruction(
    instruction: String,  // Must concatenate postfixes manually
    operand1: Option<Operand>,
    argument1: Option<Argument>,
    operand2: Option<Operand>,
    argument2: Option<Argument>,
    comment: Option<String>,
)
{
    print!("{}", instruction);

    if let Some(op1) = operand1
    {
        print!(" {}", op1);
    }

    if let Some(arg1) = argument1
    {
        match arg1
        {
            Argument::Index16(_index) => print!("{}", arg1),
            Argument::Index32(_index) => print!("{}", arg1),
            Argument::Index64(_index) => print!("{}", arg1),
            _ => print!(" {}", arg1),
        }
    }

    if operand2.is_some() || argument2.is_some()
    {
        print!(",");
    }

    if let Some(ref op2) = operand2
    {
        print!(" {}", op2);
    }

    if let Some(arg2) = argument2
    {
        match arg2
        {
            Argument::Index16(_index) =>
            {
                if operand2.is_none() { print!(" "); }
                print!("{}", arg2)
            }

            Argument::Index32(_index) =>
            {
                if operand2.is_none() { print!(" "); }
                print!("{}", arg2)
            }

            Argument::Index64(_index) =>
            {
                if operand2.is_none() { print!(" "); }
                print!("{}", arg2)
            }

            _ => print!(" {}", arg2),
        }
    }

    // TODO(pbz): Adhere to a column so they line up
    if let Some(line_comment) = comment
    {
        print!("  ;; {}", line_comment);
    }

    println!("");
}

pub struct NaturalIndex
{
    pub value: u64,
    pub sign: i8,
    pub constant: u64,
    pub natural: u64,
    pub offset: i64,
}

const SIZE_OF_VOID_PTR: u16 = 8;
const HEADER_SIZE: usize = 4;

impl NaturalIndex
{
    /// It is critical that the right method be selected per index size.
    /// Do not use `from_u64()` for a 16 bit value.
    fn from_u16(value: u16) -> Self
    {
        const ENCODING_SIZE: u16 = 2;

        let bits = bits_u16(value);
        let sign = if bits[0] { -1i64 } else { 1i64 };
        let width_base = bits_to_byte_u16(&bits[1 .. 4]);
        let actual_width = width_base * ENCODING_SIZE;
        let natural = bits_to_byte_u16(
            &bits[bits.len() - actual_width as usize ..]
        );
        let constant = bits_to_byte_u16(
            &bits[HEADER_SIZE .. bits.len() - actual_width as usize]
        );
        let offset = sign * (constant + natural * SIZE_OF_VOID_PTR) as i64;

        Self
        {
            value: value as u64,
            sign: sign as i8,
            constant: constant as u64,
            natural: natural as u64,
            offset: offset as i64
        }
    }

    /// It is critical that the right method be selected per index size.
    /// Do not use `from_u64()` for a 16 bit value.
    fn from_u32(value: u32) -> Self
    {
        const ENCODING_SIZE: u32 = 4;

        let bits = bits_u32(value);
        let sign = if bits[0] { -1i64 } else { 1i64 };
        let width_base = bits_to_byte_u32(&bits[1 .. 4]);
        let actual_width = width_base * ENCODING_SIZE;
        let natural = bits_to_byte_u32(
            &bits[bits.len() - actual_width as usize ..]
        );
        let constant = bits_to_byte_u32(
            &bits[HEADER_SIZE .. bits.len() - actual_width as usize]
        );
        let offset = {
            sign * (constant + natural * SIZE_OF_VOID_PTR as u32) as i64
        };

        Self
        {
            value: value as u64,
            sign: sign as i8,
            constant: constant as u64,
            natural: natural as u64,
            offset: offset as i64
        }
    }

    /// It is critical that the right method be selected per index size.
    /// Do not use `from_u64()` for a 16 bit value.
    fn from_u64(value: u64) -> Self
    {
        const ENCODING_SIZE: u64 = 8;

        let bits = bits_u64(value);
        let sign = if bits[0] { -1i64 } else { 1i64 };
        let width_base = bits_to_byte_u64(&bits[1 .. 4]);
        let actual_width = width_base * ENCODING_SIZE;
        let natural = bits_to_byte_u64(
            &bits[bits.len() - actual_width as usize ..]
        );
        let constant = bits_to_byte_u64(
            &bits[HEADER_SIZE .. bits.len() - actual_width as usize]
        );
        let offset = {
            sign * (constant + natural * SIZE_OF_VOID_PTR as u64) as i64
        };

        Self
        {
            value: value,
            sign: sign as i8,
            constant: constant,
            natural: natural,
            offset: offset as i64
        }
    }
}

impl std::fmt::Display for NaturalIndex
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(
            f,
            "({}{}, {}{})",
            if self.sign < 0 { "-" } else { "+" },
            self.natural,
            if self.sign < 0 { "-" } else { "+" },
            self.constant
        )
    }
}

#[derive(Debug)]
enum OpCode
{
    ADD = 0x0C,
    AND = 0x14,
    ASHR = 0x19,
    BREAK = 0x00,
    CALL = 0x03,
    CMPeq = 0x05,
    CMPlte = 0x06,
    CMPgte = 0x07,
    CMPulte = 0x08,
    CMPugte = 0x09,
    CMPIeq = 0x2D,
    CMPIlte = 0x2E,
    CMPIgte = 0x2F,
    CMPIulte = 0x30,
    CMPIugte = 0x31,
    DIV = 0x10,
    DIVU = 0x11,
    EXTNDB = 0x1A,
    EXTNDD = 0x1C,
    EXTNDW = 0x1B,
    JMP = 0x01,
    JMP8 = 0x02,
    LOADSP = 0x29,
    MOD = 0x12,
    MODU = 0x13,
    MOVbw = 0x1D,
    MOVww = 0x1E,
    MOVdw = 0x1F,
    MOVqw = 0x20,
    MOVbd = 0x21,
    MOVwd = 0x22,
    MOVdd = 0x23,
    MOVqd = 0x24,
    MOVqq = 0x28,
    MOVI = 0x37,
    MOVIn = 0x38,
    MOVnw = 0x32,
    MOVnd = 0x33,
    MOVREL = 0x39,
    MOVsnw = 0x25,
    MOVsnd = 0x26,
    MUL = 0x0E,
    MULU = 0x0F,
    NEG = 0x0B,
    NOT = 0x0A,
    OR = 0x15,
    POP = 0x2C,
    POPn = 0x36,
    PUSH = 0x2B,
    PUSHn = 0x35,
    RET = 0x04,
    SHL = 0x17,
    SHR = 0x18,
    STORESP = 0x2A,
    SUB = 0x0D,
    XOR = 0x16
}

impl std::convert::TryFrom<u8> for OpCode
{
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error>
    {
        match v
        {
            x if x == Self::ADD as u8 => Ok(Self::ADD),
            x if x == Self::AND as u8 => Ok(Self::AND),
            x if x == Self::ASHR as u8 => Ok(Self::ASHR),
            x if x == Self::BREAK as u8 => Ok(Self::BREAK),
            x if x == Self::CALL as u8 => Ok(Self::CALL),
            x if x == Self::CMPeq as u8 => Ok(Self::CMPeq),
            x if x == Self::CMPlte as u8 => Ok(Self::CMPlte),
            x if x == Self::CMPgte as u8 => Ok(Self::CMPgte),
            x if x == Self::CMPulte as u8 => Ok(Self::CMPulte),
            x if x == Self::CMPugte as u8 => Ok(Self::CMPugte),
            x if x == Self::CMPIeq as u8 => Ok(Self::CMPIeq),
            x if x == Self::CMPIlte as u8 => Ok(Self::CMPIlte),
            x if x == Self::CMPIgte as u8 => Ok(Self::CMPIgte),
            x if x == Self::CMPIulte as u8 => Ok(Self::CMPIulte),
            x if x == Self::CMPIugte as u8 => Ok(Self::CMPIugte),
            x if x == Self::DIV as u8 => Ok(Self::DIV),
            x if x == Self::DIVU as u8 => Ok(Self::DIVU),
            x if x == Self::EXTNDB as u8 => Ok(Self::EXTNDB),
            x if x == Self::EXTNDD as u8 => Ok(Self::EXTNDD),
            x if x == Self::EXTNDW as u8 => Ok(Self::EXTNDW),
            x if x == Self::JMP as u8 => Ok(Self::JMP),
            x if x == Self::JMP8 as u8 => Ok(Self::JMP8),
            x if x == Self::LOADSP as u8 => Ok(Self::LOADSP),
            x if x == Self::MOD as u8 => Ok(Self::MOD),
            x if x == Self::MODU as u8 => Ok(Self::MODU),
            x if x == Self::MOVbw as u8 => Ok(Self::MOVbw),
            x if x == Self::MOVww as u8 => Ok(Self::MOVww),
            x if x == Self::MOVdw as u8 => Ok(Self::MOVdw),
            x if x == Self::MOVqw as u8 => Ok(Self::MOVqw),
            x if x == Self::MOVbd as u8 => Ok(Self::MOVbd),
            x if x == Self::MOVwd as u8 => Ok(Self::MOVwd),
            x if x == Self::MOVdd as u8 => Ok(Self::MOVdd),
            x if x == Self::MOVqd as u8 => Ok(Self::MOVqd),
            x if x == Self::MOVqq as u8 => Ok(Self::MOVqq),
            x if x == Self::MOVI as u8 => Ok(Self::MOVI),
            x if x == Self::MOVIn as u8 => Ok(Self::MOVIn),
            x if x == Self::MOVnw as u8 => Ok(Self::MOVnw),
            x if x == Self::MOVnd as u8 => Ok(Self::MOVnd),
            x if x == Self::MOVREL as u8 => Ok(Self::MOVREL),
            x if x == Self::MOVsnw as u8 => Ok(Self::MOVsnw),
            x if x == Self::MOVsnd as u8 => Ok(Self::MOVsnd),
            x if x == Self::MUL as u8 => Ok(Self::MUL),
            x if x == Self::MULU as u8 => Ok(Self::MULU),
            x if x == Self::NEG as u8 => Ok(Self::NEG),
            x if x == Self::NOT as u8 => Ok(Self::NOT),
            x if x == Self::OR as u8 => Ok(Self::OR),
            x if x == Self::POP as u8 => Ok(Self::POP),
            x if x == Self::POPn as u8 => Ok(Self::POPn),
            x if x == Self::PUSH as u8 => Ok(Self::PUSH),
            x if x == Self::PUSHn as u8 => Ok(Self::PUSHn),
            x if x == Self::RET as u8 => Ok(Self::RET),
            x if x == Self::SHL as u8 => Ok(Self::SHL),
            x if x == Self::SHR as u8 => Ok(Self::SHR),
            x if x == Self::STORESP as u8 => Ok(Self::STORESP),
            x if x == Self::SUB as u8 => Ok(Self::SUB),
            x if x == Self::XOR as u8 => Ok(Self::XOR),
            _ => Err(()),
        }
    }
}

/// Needed since stringifying the OpCode is part of application functionality.
impl std::fmt::Display for OpCode
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "{:?}", self)
    }
}

impl OpCode
{
    /// Bytes are read from left to right. Bits are read from right to left.
    fn disassemble<T: Iterator<Item=u8>>(bytes: &mut T) -> Option<()>
    {
        let byte0 = if let Some(byte) = bytes.next()
        {
            byte
        }
        else
        {
            return None;
        };

        // * Using reverse number parsing to make indexing the individual bits
        // * easier since the UEFI spec specifies them in reverse.

        let byte0_bits = bits_rev(byte0);
        let op_value = bits_to_byte_rev(&byte0_bits[0 ..= 5]);
        let op: OpCode = op_value.try_into().expect(
            format!("Invalid OpCode: {}", op_value).as_str()
        );

        match op
        {
            // 1. INSTRUCTION (RET)
            OpCode::RET => parse_instruction1(bytes, byte0_bits, op),

            OpCode::JMP8
            | OpCode::BREAK =>
            {
                // 2. INSTRUCTION ARGUMENT (BREAK)
                parse_instruction2(bytes, byte0_bits, op)
            }

            OpCode::CALL
            | OpCode::JMP
            | OpCode::PUSH
            | OpCode::PUSHn
            | OpCode::POP
            | OpCode::POPn =>
            {
                // 3. INSTRUCTION OP1 ARGUMENT (CALL)
                parse_instruction3(bytes, byte0_bits, op)
            }

            OpCode::LOADSP
            | OpCode::STORESP =>
            {
                // 4. INSTRUCTION OP1, OP2 (STORESP)
                parse_instruction4(bytes, byte0_bits, op)
            }

            OpCode::CMPIeq
            | OpCode::CMPIlte
            | OpCode::CMPIgte
            | OpCode::CMPIulte
            | OpCode::CMPIugte
            | OpCode::MOVI
            | OpCode::MOVIn
            | OpCode::MOVREL =>
            {
                // 5. INSTRUCTION OP1 ARGUMENT, ARGUMENT (CMPI)
                parse_instruction5(bytes, byte0_bits, op)
            }

            OpCode::ADD
            | OpCode::AND
            | OpCode::ASHR
            | OpCode::CMPeq
            | OpCode::CMPlte
            | OpCode::CMPgte
            | OpCode::CMPulte
            | OpCode::CMPugte
            | OpCode::DIV
            | OpCode::DIVU
            | OpCode::EXTNDB
            | OpCode::EXTNDD
            | OpCode::EXTNDW
            | OpCode::MOD
            | OpCode::MODU
            | OpCode::MUL
            | OpCode::MULU
            | OpCode::NEG
            | OpCode::NOT
            | OpCode::OR
            | OpCode::SHL
            | OpCode::SHR
            | OpCode::SUB
            | OpCode::XOR =>
            {
                // 6. INSTRUCTION OP1, OP2 ARGUMENT
                // (16 bit optional index/immediate) (MUL)
                parse_instruction6(bytes, byte0_bits, op)
            }

            OpCode::MOVnw
            | OpCode::MOVnd
            | OpCode::MOVbw
            | OpCode::MOVww
            | OpCode::MOVdw
            | OpCode::MOVqw
            | OpCode::MOVbd
            | OpCode::MOVwd
            | OpCode::MOVdd
            | OpCode::MOVqd
            | OpCode::MOVqq
            | OpCode::MOVsnw
            | OpCode::MOVsnd =>
            {
                // 7. INSTRUCTION OP1 ARGUMENT, OP2 ARGUMENT (MOV)
                parse_instruction7(bytes, byte0_bits, op)
            }
        }
    }
}

// fn bits(byte: u8) -> [bool; 8]
// {
//     let mut bits = [false; 8];

//     for i in 0 .. 8
//     {
//         if byte & 2u8.pow(i) > 0
//         {
//             bits[(bits.len() - 1) - i as usize] = true;
//         }
//     }

//     bits
// }


// fn bits_u8(byte: u8) -> [bool; 8]
// {
//     let mut bits = [false; 8];

//     for i in 0 .. 8
//     {
//         if byte & 2u8.pow(i) > 0
//         {
//             bits[(bits.len() - 1) - i as usize] = true;
//         }
//     }

//     bits
// }

// fn bits_to_byte_u8(bits: &[bool]) -> u8
// {
//     let mut byte = 0;

//     for (i, bit) in bits.iter().rev().enumerate()
//     {
//         if *bit
//         {
//             // byte += 2u8.pow((bits.len() - 1 - i) as u32);
//             byte += 2u8.pow((i) as u32);
//         }
//     }
//     byte
// }

fn bits_u16(byte: u16) -> [bool; 16]
{
    let mut bits = [false; 16];

    for i in 0 .. 16
    {
        if byte & 2u16.pow(i) > 0
        {
            bits[(bits.len() - 1) - i as usize] = true;
        }
    }

    bits
}

fn bits_to_byte_u16(bits: &[bool]) -> u16
{
    let mut byte = 0;

    for (i, bit) in bits.iter().rev().enumerate()
    {
        if *bit
        {
            byte += 2u16.pow((i) as u32);
        }
    }
    byte
}

fn bits_u32(byte: u32) -> [bool; 32]
{
    let mut bits = [false; 32];

    for i in 0 .. 32
    {
        if byte & 2u32.pow(i) > 0
        {
            bits[(bits.len() - 1) - i as usize] = true;
        }
    }

    bits
}

fn bits_to_byte_u32(bits: &[bool]) -> u32
{
    let mut byte = 0;

    for (i, bit) in bits.iter().rev().enumerate()
    {
        if *bit
        {
            byte += 2u32.pow((i) as u32);
        }
    }
    byte
}

fn bits_u64(byte: u64) -> [bool; 64]
{
    let mut bits = [false; 64];

    for i in 0 .. 64
    {
        if byte & 2u64.pow(i) > 0
        {
            bits[(bits.len() - 1) - i as usize] = true;
        }
    }

    bits
}

fn bits_to_byte_u64(bits: &[bool]) -> u64
{
    let mut byte = 0;

    for (i, bit) in bits.iter().rev().enumerate()
    {
        if *bit
        {
            byte += 2u64.pow((i) as u32);
        }
    }
    byte
}

/// Returns the bits of a byte in reverse so that indexing works as expected.
fn bits_rev(byte: u8) -> [bool; 8]
{
    let mut bits = [false; 8];

    for i in 0 .. 8
    {
        if byte & 2u8.pow(i) > 0
        {
            bits[i as usize] = true;
        }
    }

    bits
}

/// Converts a slice of bits sorted in reverse to a byte.
fn bits_to_byte_rev(bits: &[bool]) -> u8
{
    let mut byte = 0;

    for (i, bit) in bits.iter().enumerate()
    {
        if *bit
        {
            byte += 2u8.pow((i) as u32);
        }
    }
    byte
}

/// Reads in an EFI Bytecode file from STDIN and prints the disassembly.
fn main()
{
    let mut show_help = true;
    for bytecode_file in std::env::args().skip(1).take(1)
    {
        show_help = false;

        let file = std::fs::File::open(bytecode_file.clone()).expect(
            format!("File {} does not exist", bytecode_file).as_str()
        );
        let mut bytes = file.bytes().map(|b| b.unwrap());

        loop
        {
            if OpCode::disassemble(&mut bytes).is_none()
            {
                break;
            }
        }

        // TODO(pbz): Bytes can be left over in the instruction. Process them.
    }

    if show_help
    {
        println!(
            "Spore - Disassembler for UEFI Bytecode\nUsage: spore <FILENAME>"
        );
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_bits_to_byte()
    {
        assert_eq!(bits_to_byte_u8(&[true, false]), 2u8);
        assert_eq!(bits_to_byte_u8(&[false, true, false]), 2u8);
        assert_eq!(bits_to_byte_u8(&[true, false, false]), 4u8);
        assert_eq!(bits_to_byte_u8(&[true, false, false, false]), 8u8);
        assert_eq!(bits_to_byte_u8(&[true, false, false, true]), 9u8);
        assert_eq!(bits_to_byte_u8(&[true, false, true, true]), 11u8);

        assert_eq!(bits_to_byte_u32(&[true, false]), 2u32);
        assert_eq!(bits_to_byte_u32(&[false, true, false]), 2u32);
        assert_eq!(bits_to_byte_u32(&[true, false, false]), 4u32);
        assert_eq!(bits_to_byte_u32(&[true, false, false, false]), 8u32);
        assert_eq!(bits_to_byte_u32(&[true, false, false, true]), 9u32);
        assert_eq!(bits_to_byte_u32(&[true, false, true, true]), 11u32);

        assert_eq!(bits_to_byte_u64(&[true, false]), 2u64);
        assert_eq!(bits_to_byte_u64(&[false, true, false]), 2u64);
        assert_eq!(bits_to_byte_u64(&[true, false, false]), 4u64);
        assert_eq!(bits_to_byte_u64(&[true, false, false, false]), 8u64);
        assert_eq!(bits_to_byte_u64(&[true, false, false, true]), 9u64);
        assert_eq!(bits_to_byte_u64(&[true, false, true, true]), 11u64);
    }

    #[test]
    fn test_bits()
    {
        assert_eq!(
            bits_u8(2u8),
            [false, false, false, false, false, false, true, false]
        );

        assert_eq!(
            bits_u8(4u8),
            [false, false, false, false, false, true, false, false]
        );

        assert_eq!(
            bits_u8(0x32u8),
            [false, false, true, true, false, false, true, false]
        );
    }

    #[test]
    fn test_natural_indexing()
    {
        let index = NaturalIndex::from_u16(4161);
        assert_eq!(index.constant, 16u64);
        assert_eq!(index.natural, 1u64);
        assert_eq!(index.offset, 24i64);

        let index = NaturalIndex::from_u16(4114);
        assert_eq!(index.constant, 4u64);
        assert_eq!(index.natural, 2u64);
        assert_eq!(index.offset, 20i64);

        let index = NaturalIndex::from_u16(8581);
        assert_eq!(index.constant, 24u64);
        assert_eq!(index.natural, 5u64);
        assert_eq!(index.offset, 64i64);

        let index = NaturalIndex::from_u32(805324752);
        assert_eq!(index.constant, 4u64);
        assert_eq!(index.natural, 2000u64);
        assert_eq!(index.offset, 16004i64);

        let index = NaturalIndex::from_u32(111111);
        assert_eq!(index.constant, 111111u64);
        assert_eq!(index.natural, 0u64);
        assert_eq!(index.offset, 111111i64);

        let index = NaturalIndex::from_u64(2305843035428095952);
        assert_eq!(index.constant, 400000u64);
        assert_eq!(index.natural, 2000u64);
        assert_eq!(index.offset, 416000i64);

        let index = NaturalIndex::from_u32(591751049);
        assert_eq!(index.constant, 214375u64);
        assert_eq!(index.natural, 137u64);
        assert_eq!(index.offset, 215471i64);

        let index = NaturalIndex::from_u64(11529215072282871760);
        assert_eq!(index.sign, -1i8);
        assert_eq!(index.constant, 400000u64);
        assert_eq!(index.natural, 2000u64);
        assert_eq!(index.offset, -416000i64);
    }
}
