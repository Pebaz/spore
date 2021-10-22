use crate::opcode::*;
use crate::operand::*;
use crate::argument::*;
use crate::bits::*;
use crate::options::Options;
use crate::theme::*;

pub fn parse_instruction1<W: std::io::Write, T: Iterator<Item=u8>>(
    writer: &mut W,
    options: &Options,
    _bytes: &mut T,
    _byte0_bits: [bool; 8],
    op: OpCode,
) -> Option<()>
{
    disassemble_instruction(
        writer,
        // format!("{}", op).truecolor(BLUE.0, BLUE.1, BLUE.2).to_string(),
        color_opcode(format!("{}", op), options),
        None,
        None,
        None,
        None,
        None
    );

    Some(())
}

pub fn parse_instruction2<W: std::io::Write, T: Iterator<Item=u8>>(
    writer: &mut W,
    options: &Options,
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
        writer,
        // name.truecolor(BLUE.0, BLUE.1, BLUE.2).to_string(),
        color_opcode(format!("{}", name), options),
        None,
        Some(arg1),
        None,
        None,
        None
    );

    Some(())
}

pub fn parse_instruction3<W: std::io::Write, T: Iterator<Item=u8>>(
    writer: &mut W,
    options: &Options,
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
                        // TODO(pbz): These fail hard. Print better message
                        // ? Perhaps a 16 bit was passed to a 32 bit?
                        // ? "32-bit value expected, found less than that"
                        // value[i] = bytes.next().expect(
                        //     format!("Unexpected end of byte stream while processing instruction: {}", op).as_str()
                        // );
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
        writer,
        // format!("{}", name).truecolor(BLUE.0, BLUE.1, BLUE.2).to_string(),
        color_opcode(format!("{}", name), options),
        op1,
        arg1,
        op2,
        arg2,
        comment
    );

    Some(())
}

pub fn parse_instruction4<W: std::io::Write, T: Iterator<Item=u8>>(
    writer: &mut W,
    options: &Options,
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
        writer,
        // name.truecolor(BLUE.0, BLUE.1, BLUE.2).to_string(),
        color_opcode(format!("{}", name), options),
        Some(op1),
        None,
        Some(op2),
        None,
        None
    );

    Some(())
}

pub fn parse_instruction5<W: std::io::Write, T: Iterator<Item=u8>>(
    writer: &mut W,
    options: &Options,
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
        writer,
        // name.truecolor(BLUE.0, BLUE.1, BLUE.2).to_string(),
        color_opcode(format!("{}", name), options),
        op1,
        arg1,
        None,
        arg2,
        None
    );

    Some(())
}

pub fn parse_instruction6<W: std::io::Write, T: Iterator<Item=u8>>(
    writer: &mut W,
    options: &Options,
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
        writer,
        // name.truecolor(BLUE.0, BLUE.1, BLUE.2).to_string(),
        color_opcode(format!("{}", name), options),
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

pub fn parse_instruction7<W: std::io::Write, T: Iterator<Item=u8>>(
    writer: &mut W,
    options: &Options,
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
        writer,
        // name.truecolor(BLUE.0, BLUE.1, BLUE.2).to_string(),
        color_opcode(format!("{}", name), options),
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
pub fn disassemble_instruction<W: std::io::Write>(
    writer: &mut W,
    instruction: String,  // Must concatenate postfixes manually
    operand1: Option<Operand>,
    argument1: Option<Argument>,
    operand2: Option<Operand>,
    argument2: Option<Argument>,
    comment: Option<String>,
)
{
    write!(writer, "{}", instruction).unwrap();

    if let Some(op1) = operand1
    {
        write!(writer, " {}", op1).unwrap();
    }

    if let Some(arg1) = argument1
    {
        match arg1
        {
            Argument::Index16(_index) => write!(writer, "{}", arg1).unwrap(),
            Argument::Index32(_index) => write!(writer, "{}", arg1).unwrap(),
            Argument::Index64(_index) => write!(writer, "{}", arg1).unwrap(),
            _ => write!(writer, " {}", arg1).unwrap(),
        }
    }

    if operand2.is_some() || argument2.is_some()
    {
        write!(writer, ",").unwrap();
    }

    if let Some(ref op2) = operand2
    {
        write!(writer, " {}", op2).unwrap();
    }

    if let Some(arg2) = argument2
    {
        match arg2
        {
            Argument::Index16(_index) =>
            {
                if operand2.is_none() { write!(writer, " ").unwrap(); }
                write!(writer, "{}", arg2).unwrap();
            }

            Argument::Index32(_index) =>
            {
                if operand2.is_none() { write!(writer, " ").unwrap(); }
                write!(writer, "{}", arg2).unwrap();
            }

            Argument::Index64(_index) =>
            {
                if operand2.is_none() { write!(writer, " ").unwrap(); }
                write!(writer, "{}", arg2).unwrap();
            }

            _ => write!(writer, " {}", arg2).unwrap(),
        }
    }

    // TODO(pbz): Adhere to a column so they line up
    if let Some(line_comment) = comment
    {
        write!(writer, "  ;; {}", line_comment).unwrap();
    }

    writeln!(writer, "").unwrap();
}
