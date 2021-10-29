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

use arrayvec::ArrayVec;
use crate::opcode::*;
use crate::operand::*;
use crate::argument::*;
use crate::bits::*;
use crate::options::Options;
use crate::theme::*;

fn read_value<T: Iterator<Item=u8>, const WIDTH: usize>(
    bytes: &mut T
) -> Result<[u8; WIDTH], String>
{
    let mut value = [0u8; WIDTH];

    for i in 0 .. value.len()
    {
        value[i] = bytes.next().ok_or("Unexpected end of byte stream")?;
    }

    Ok(value)
}

pub fn parse_instruction1<W: std::io::Write, T: Iterator<Item=u8>>(
    writer: &mut W,
    options: &Options,
    _bytes: &mut T,
    byte0: u8,
    _byte0_bits: [bool; 8],
    op: OpCode,
) -> Result<(), String>
{
    let bytecode = [byte0];
    disassemble_instruction(
        writer,
        options,
        &bytecode,
        op.emit(options),
        None,
        None,
        None,
        None,
        None
    );

    Ok(())
}

pub fn parse_instruction2<W: std::io::Write, T: Iterator<Item=u8>>(
    writer: &mut W,
    options: &Options,
    bytes: &mut T,
    byte0: u8,
    byte0_bits: [bool; 8],
    op: OpCode,
) -> Result<(), String>
{
    let mut name = op.emit(options);

    let byte1 = bytes.next().ok_or("Unexpected end of bytes")?;

    let arg1 = match op
    {
        OpCode::BREAK =>
        {
            if byte1 == 0
            {
                let msg = String::from(
                    "Runaway program break (found 2 zeros in a row, BREAK 0)"
                );

                return Err(color_error(msg, options));
            }

            Argument::ImmediateU16(byte1 as u16)
        }

        OpCode::JMP8 =>
        {
            let conditional = byte0_bits[7];

            if conditional
            {
                let condition_bit_set = byte0_bits[6];
                let postfix = if condition_bit_set
                {
                    color_opcode(String::from("cs"), options)
                }
                else
                {
                    color_opcode(String::from("cc"), options)
                };

                name += &postfix;
            }

            Argument::ImmediateI16((byte1 as i8) as i16)
        }

        _ => unreachable!(),
    };

    let bytecode = [byte0, byte1];
    disassemble_instruction(
        writer,
        options,
        &bytecode,
        name,
        None,
        Some(arg1),
        None,
        None,
        None
    );

    Ok(())
}

pub fn parse_instruction3<W: std::io::Write, T: Iterator<Item=u8>>(
    writer: &mut W,
    options: &Options,
    bytes: &mut T,
    byte0: u8,
    byte0_bits: [bool; 8],
    op: OpCode,
) -> Result<(), String>
{
    let mut name = op.emit(options);
    let mut postfix = String::with_capacity(5);
    let immediate_data_present = byte0_bits[7];
    let is_64_bit = byte0_bits[6];  // Not used by PUSHn & POPn

    let byte1 = bytes.next().ok_or("Unexpected end of bytes")?;
    let byte1_bits = bits_rev(byte1);

    let mut bytecode = ArrayVec::<_, 18>::new();
    bytecode.push(byte0);
    bytecode.push(byte1);

    match op
    {
        OpCode::CALL
        | OpCode::JMP
        | OpCode::PUSH
        | OpCode::POP =>
        {
            let width_postfix = if is_64_bit
            {
                color_x64(String::from("64"), options)
            }
            else
            {
                color_x32(String::from("32"), options)
            };

            postfix += &width_postfix;
        }

        _ => (),
    }

    let (op1, arg1, op2, arg2, comment) = match op
    {
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

                let value = read_value::<T, 8>(bytes)?;
                bytecode.extend(value.iter().cloned());

                Some(Argument::ImmediateI64(i64::from_le_bytes(value)))
            }
            else
            {
                postfix += if is_relative_address { "" } else { "a" };

                let arg = if immediate_data_present
                {
                    let value = read_value::<T, 4>(bytes)?;


                    bytecode.extend(value.iter().cloned());

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
                let value = read_value::<T, 8>(bytes)?;
                bytecode.extend(value.iter().cloned());

                Some(Argument::ImmediateI64(i64::from_le_bytes(value)))
            }
            else
            {
                let arg = if immediate_data_present
                {
                    let value = read_value::<T, 4>(bytes)?;
                    bytecode.extend(value.iter().cloned());

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
                let value = read_value::<T, 2>(bytes)?;
                bytecode.extend(value.iter().cloned());

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
        options,
        &bytecode,
        name,
        op1,
        arg1,
        op2,
        arg2,
        comment
    );

    Ok(())
}

pub fn parse_instruction4<W: std::io::Write, T: Iterator<Item=u8>>(
    writer: &mut W,
    options: &Options,
    bytes: &mut T,
    byte0: u8,
    _byte0_bits: [bool; 8],
    op: OpCode,
) -> Result<(), String>
{
    let name = op.emit(options);

    let byte1 = bytes.next().ok_or("Unexpected end of bytes")?;
    let byte1_bits = bits_rev(byte1);
    let operand1_value = bits_to_byte_rev(&byte1_bits[0 ..= 2]);
    let operand2_value = bits_to_byte_rev(&byte1_bits[4 ..= 6]);

    let mut bytecode = ArrayVec::<_, 18>::new();
    bytecode.push(byte0);
    bytecode.push(byte1);

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
        options,
        &bytecode,
        name,
        Some(op1),
        None,
        Some(op2),
        None,
        None
    );

    Ok(())
}

pub fn parse_instruction5<W: std::io::Write, T: Iterator<Item=u8>>(
    writer: &mut W,
    options: &Options,
    bytes: &mut T,
    byte0: u8,
    byte0_bits: [bool; 8],
    op: OpCode,
) -> Result<(), String>
{
    let mut name = op.emit(options);
    let mut postfixes = String::with_capacity(7);
    let byte1 = bytes.next().ok_or("Unexpected end of byte stream")?;
    let byte1_bits = bits_rev(byte1);

    let mut bytecode = ArrayVec::<_, 18>::new();
    bytecode.push(byte0);
    bytecode.push(byte1);

    let (op1, arg1, arg2) = match op
    {
        OpCode::MOVI =>
        {
            let move_width = bits_to_byte_rev(&byte1_bits[4 ..= 5]);
            let postfix = match move_width
            {
                0 => color_x8(String::from("b"), options),
                1 => color_x16(String::from("w"), options),
                2 => color_x32(String::from("d"), options),
                3 => color_x64(String::from("q"), options),
                _ => unreachable!(),
            };

            postfixes += &postfix;

            let immediate_data_width = bits_to_byte_rev(&byte0_bits[6 ..= 7]);
            let postfix = match immediate_data_width
            {
                1 => color_x16(String::from("w"), options),
                2 => color_x32(String::from("d"), options),
                3 => color_x64(String::from("q"), options),
                _ => unreachable!(),
            };

            postfixes += &postfix;

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
                let value = read_value::<T, 2>(bytes)?;
                bytecode.extend(value.iter().cloned());

                let arg = if operand1_is_indirect
                {
                    Argument::Index16(u16::from_le_bytes(value))
                }
                else
                {
                    let msg = format!(
                        "Immediate data not supported for {}",
                        op.emit(options)
                    );

                    return Err(color_error(msg, options));
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
                        let value = read_value::<T, 2>(bytes)?;
                        bytecode.extend(value.iter().cloned());

                        Some(Argument::ImmediateI16(i16::from_le_bytes(value)))
                    }

                    2 =>  // 32 bit
                    {
                        let value = read_value::<T, 4>(bytes)?;
                        bytecode.extend(value.iter().cloned());

                        Some(Argument::ImmediateI32(i32::from_le_bytes(value)))
                    }

                    3 =>  // 64 bit
                    {
                        let value = read_value::<T, 8>(bytes)?;
                        bytecode.extend(value.iter().cloned());

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
            name = color_opcode(String::from("CMPI"), options);

            let postfix = if comparison_is_64_bit
            {
                color_x64(String::from("64"), options)
            }
            else
            {
                color_x32(String::from("32"), options)
            };

            name += &postfix;

            let postfix = if immediate_data_is_32_bit
            {
                color_x32(String::from("d"), options)
            }
            else
            {
                color_x16(String::from("w"), options)
            };

            name += &postfix;

            let postfix = match op
            {
                OpCode::CMPIeq => color_opcode(String::from("eq"), options),
                OpCode::CMPIlte => color_opcode(String::from("lte"), options),
                OpCode::CMPIgte => color_opcode(String::from("gte"), options),
                OpCode::CMPIulte =>
                {
                    color_opcode(String::from("ulte"), options)
                }

                OpCode::CMPIugte =>
                {
                    color_opcode(String::from("ugte"), options)
                }

                _ => unreachable!(),
            };

            name += &postfix;

            let operand1_index_present = byte1_bits[4];
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
                let value = read_value::<T, 2>(bytes)?;
                bytecode.extend(value.iter().cloned());

                let arg = if operand1_is_indirect
                {
                    Argument::Index16(u16::from_le_bytes(value))
                }
                else
                {
                    let msg = format!(
                        "Immediate data not supported for {}",
                        op.emit(options)
                    );

                    return Err(color_error(msg, options));
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
                    let value = read_value::<T, 4>(bytes)?;
                    bytecode.extend(value.iter().cloned());

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
                    let value = read_value::<T, 2>(bytes)?;
                    bytecode.extend(value.iter().cloned());

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
            let postfix = match operand2_index_width
            {
                1 => color_x16(String::from("w"), options),
                2 => color_x32(String::from("d"), options),
                3 => color_x64(String::from("q"), options),
                _ => unreachable!(),
            };

            postfixes += &postfix;

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
                let value = read_value::<T, 2>(bytes)?;
                bytecode.extend(value.iter().cloned());

                let arg = if operand1_is_indirect
                {
                    Argument::Index16(u16::from_le_bytes(value))
                }
                else
                {
                    let msg = format!(
                        "Immediate data not supported for {}",
                        op.emit(options)
                    );

                    return Err(color_error(msg, options));
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
                        let value = read_value::<T, 2>(bytes)?;
                        bytecode.extend(value.iter().cloned());

                        Some(Argument::Index16(u16::from_le_bytes(value)))
                    }

                    2 =>  // 32 bit
                    {
                        let value = read_value::<T, 4>(bytes)?;
                        bytecode.extend(value.iter().cloned());

                        Some(Argument::Index32(u32::from_le_bytes(value)))
                    }

                    3 =>  // 64 bit
                    {
                        let value = read_value::<T, 8>(bytes)?;
                        bytecode.extend(value.iter().cloned());

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
            let postfix = match immediate_data_width
            {
                1 => color_x16(String::from("w"), options),
                2 => color_x32(String::from("d"), options),
                3 => color_x64(String::from("q"), options),
                _ => unreachable!(),
            };

            postfixes += &postfix;

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
                let value = read_value::<T, 2>(bytes)?;
                bytecode.extend(value.iter().cloned());

                let arg = if operand1_is_indirect
                {
                    Argument::Index16(u16::from_le_bytes(value))
                }
                else
                {
                    let msg = format!(
                        "Immediate data not supported for {}",
                        op.emit(options)
                    );

                    return Err(color_error(msg, options));
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
                        let value = read_value::<T, 2>(bytes)?;
                        bytecode.extend(value.iter().cloned());

                        Some(Argument::ImmediateI16(i16::from_le_bytes(value)))
                    }

                    2 =>  // 32 bit
                    {
                        let value = read_value::<T, 4>(bytes)?;
                        bytecode.extend(value.iter().cloned());

                        Some(Argument::ImmediateI32(i32::from_le_bytes(value)))
                    }

                    3 =>  // 64 bit
                    {
                        let value = read_value::<T, 8>(bytes)?;
                        bytecode.extend(value.iter().cloned());

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
        options,
        &bytecode,
        name,
        op1,
        arg1,
        None,
        arg2,
        None
    );

    Ok(())
}

pub fn parse_instruction6<W: std::io::Write, T: Iterator<Item=u8>>(
    writer: &mut W,
    options: &Options,
    bytes: &mut T,
    byte0: u8,
    byte0_bits: [bool; 8],
    op: OpCode,
) -> Result<(), String>
{
    let mut name = op.emit(options);
    let immediate_data_present = byte0_bits[7];
    let is_64_bit = byte0_bits[6];
    let postfix = if is_64_bit
    {
        color_x64(String::from("64"), options)
    }
    else
    {
        color_x32(String::from("32"), options)
    };

    name += &postfix;

    let byte1 = bytes.next().ok_or("Unexpected end of bytes")?;
    let byte1_bits = bits_rev(byte1);
    let operand1_is_indirect = byte1_bits[3];
    let operand1_value = bits_to_byte_rev(&byte1_bits[0 ..= 2]);
    let operand2_is_indirect = byte1_bits[7];
    let operand2_value = bits_to_byte_rev(&byte1_bits[4 ..= 6]);

    let mut bytecode = ArrayVec::<_, 18>::new();
    bytecode.push(byte0);
    bytecode.push(byte1);

    let op1_x16_index_or_immediate =
    {
        if immediate_data_present
        {
            let value = read_value::<T, 2>(bytes)?;
            bytecode.extend(value.iter().cloned());

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
        options,
        &bytecode,
        name,
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

    Ok(())
}

pub fn parse_instruction7<W: std::io::Write, T: Iterator<Item=u8>>(
    writer: &mut W,
    options: &Options,
    bytes: &mut T,
    byte0: u8,
    byte0_bits: [bool; 8],
    op: OpCode,
) -> Result<(), String>
{
    let operand1_index_present = byte0_bits[7];
    let operand2_index_present = byte0_bits[6];

    let byte1 = bytes.next().ok_or("Unexpected end of bytes")?;
    let byte1_bits = bits_rev(byte1);
    let operand1_is_indirect = byte1_bits[3];
    let operand1_value = bits_to_byte_rev(&byte1_bits[0 ..= 2]);
    let operand2_is_indirect = byte1_bits[7];
    let operand2_value = bits_to_byte_rev(&byte1_bits[4 ..= 6]);

    let mut bytecode = ArrayVec::<_, 18>::new();
    bytecode.push(byte0);
    bytecode.push(byte1);

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
                            let value = read_value::<T, 2>(bytes)?;
                            bytecode.extend(value.iter().cloned());

                            Argument::Index16(u16::from_le_bytes(value))
                        }

                        OpCode::MOVsnd =>  // 32 bit
                        {
                            let value = read_value::<T, 4>(bytes)?;
                            bytecode.extend(value.iter().cloned());

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
                            let value = read_value::<T, 2>(bytes)?;
                            bytecode.extend(value.iter().cloned());

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
                            let value = read_value::<T, 4>(bytes)?;
                            bytecode.extend(value.iter().cloned());

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
                            let value = read_value::<T, 2>(bytes)?;
                            bytecode.extend(value.iter().cloned());

                            Argument::Index16(u16::from_le_bytes(value))
                        }

                        OpCode::MOVnd =>  // 32 bit
                        {
                            let value = read_value::<T, 4>(bytes)?;
                            bytecode.extend(value.iter().cloned());

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
                            let value = read_value::<T, 2>(bytes)?;
                            bytecode.extend(value.iter().cloned());

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
                            let value = read_value::<T, 4>(bytes)?;
                            bytecode.extend(value.iter().cloned());

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
                            let value = read_value::<T, 2>(bytes)?;
                            bytecode.extend(value.iter().cloned());

                            Argument::Index16(u16::from_le_bytes(value))
                        }

                        OpCode::MOVbd
                        | OpCode::MOVwd
                        | OpCode::MOVdd
                        | OpCode::MOVqd =>  // 32 bit
                        {
                            let value = read_value::<T, 4>(bytes)?;
                            bytecode.extend(value.iter().cloned());

                            Argument::Index32(u32::from_le_bytes(value))
                        }

                        OpCode::MOVqq =>  // 64 bit
                        {
                            let value = read_value::<T, 8>(bytes)?;
                            bytecode.extend(value.iter().cloned());

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
                            let value = read_value::<T, 2>(bytes)?;
                            bytecode.extend(value.iter().cloned());

                            Argument::Index16(u16::from_le_bytes(value))
                        }

                        OpCode::MOVbd
                        | OpCode::MOVwd
                        | OpCode::MOVdd
                        | OpCode::MOVqd =>  // 32 bit
                        {
                            let value = read_value::<T, 4>(bytes)?;
                            bytecode.extend(value.iter().cloned());

                            Argument::Index32(u32::from_le_bytes(value))
                        }

                        OpCode::MOVqq =>  // 64 bit
                        {
                            let value = read_value::<T, 8>(bytes)?;
                            bytecode.extend(value.iter().cloned());

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

    let mut name = format!("{:?}", op);
    let mut chars = name.chars();
    let indices_present = operand1_index_present || operand2_index_present;

    let postfix = match op
    {
        OpCode::MOVnw
        | OpCode::MOVsnw =>
        {
            // If indices are present, keep them
            let move_width = if indices_present
            {
                String::from(chars.next_back().unwrap())
            }
            else  // Remove it to get to the guaranteed move width
            {
                chars.next_back().unwrap();
                String::from("")
            };

            // COLOR IT
            color_x16(move_width, options)
        }

        OpCode::MOVnd
        | OpCode::MOVsnd =>
        {
            // If indices are present, keep them
            let move_width = if indices_present
            {
                String::from(chars.next_back().unwrap())
            }
            else  // Remove it to get to the guaranteed move width
            {
                chars.next_back().unwrap();
                String::from("")
            };

            // COLOR IT

            color_x32(move_width, options)
        }

        OpCode::MOVbw
        | OpCode::MOVww
        | OpCode::MOVdw
        | OpCode::MOVqw
        | OpCode::MOVbd
        | OpCode::MOVwd
        | OpCode::MOVdd
        | OpCode::MOVqd
        | OpCode::MOVqq =>
        {
            // If indices are present, keep them
            let index_width = if indices_present
            {
                let width = String::from(chars.next_back().unwrap());

                match op
                {
                    OpCode::MOVbw
                    | OpCode::MOVww
                    | OpCode::MOVdw
                    | OpCode::MOVqw =>
                    {
                        color_x16(width, options)
                    }

                    OpCode::MOVbd
                    | OpCode::MOVwd
                    | OpCode::MOVdd
                    | OpCode::MOVqd =>
                    {
                        color_x32(width, options)
                    }

                    OpCode::MOVqq => color_x64(width, options),

                    _ => unreachable!(),
                }
            }
            else  // Remove it to get to the guaranteed move width
            {
                chars.next_back().unwrap();
                String::from("")
            };

            let move_width = String::from(chars.next_back().unwrap());
            let move_width = match op
            {
                OpCode::MOVbw | OpCode::MOVbd => color_x8(move_width, options),

                OpCode::MOVww | OpCode::MOVwd => color_x16(move_width, options),

                OpCode::MOVdw | OpCode::MOVdd => color_x32(move_width, options),

                OpCode::MOVqw | OpCode::MOVqd | OpCode::MOVqq =>
                {
                    color_x64(move_width, options)
                }

                _ => unreachable!(),
            };

            move_width + &index_width
        }

        _ => unreachable!(),
    };

    let postfixes_removed = chars.as_str().to_string();
    name = color_opcode(postfixes_removed, options);
    name += &postfix;

    disassemble_instruction(
        writer,
        options,
        &bytecode,
        name,
        op1,
        arg1,
        op2,
        arg2,
        None
    );

    Ok(())
}

pub fn disassemble_instruction<W: std::io::Write>(
    writer: &mut W,
    options: &Options,
    bytecode: &[u8],
    instruction: String,  // Must concatenate postfixes manually
    operand1: Option<Operand>,
    argument1: Option<Argument>,
    operand2: Option<Operand>,
    argument2: Option<Argument>,
    comment: Option<String>,
)
{
    if options.bytecode
    {
        const TWO_CHARS_AND_A_SPACE: usize = 3;
        let mut bytecode_output = String::with_capacity(
            bytecode.len() * TWO_CHARS_AND_A_SPACE
        );

        for byte in bytecode.iter()
        {
            bytecode_output += format!("{:<02X?} ", byte).as_str();
        }

        bytecode_output = color_bytecode(bytecode_output, options);

        write!(writer, "{:>84} ", bytecode_output).unwrap();
    }

    if options.pad_output
    {
        write!(writer, "{}", instruction).unwrap();
    }
    else
    {
        write!(writer, "{}", instruction).unwrap();
    }

    if let Some(op1) = operand1
    {
        write!(writer, " {}", op1.emit(options)).unwrap();
    }

    if let Some(arg1) = argument1
    {
        let text = arg1.emit(options);

        match arg1
        {
            Argument::Index16(_index) => write!(writer, "{}", text).unwrap(),
            Argument::Index32(_index) => write!(writer, "{}", text).unwrap(),
            Argument::Index64(_index) => write!(writer, "{}", text).unwrap(),
            _ => write!(writer, " {}", text).unwrap(),
        }
    }

    if operand2.is_some() || argument2.is_some()
    {
        write!(writer, ",").unwrap();
    }

    if let Some(ref op2) = operand2
    {
        write!(writer, " {}", op2.emit(options)).unwrap();
    }

    if let Some(arg2) = argument2
    {
        match arg2
        {
            Argument::Index16(_index) =>
            {
                if operand2.is_none() { write!(writer, " ").unwrap(); }
                write!(writer, "{}", arg2.emit(options)).unwrap();
            }

            Argument::Index32(_index) =>
            {
                if operand2.is_none() { write!(writer, " ").unwrap(); }
                write!(writer, "{}", arg2.emit(options)).unwrap();
            }

            Argument::Index64(_index) =>
            {
                if operand2.is_none() { write!(writer, " ").unwrap(); }
                write!(writer, "{}", arg2.emit(options)).unwrap();
            }

            _ => write!(writer, " {}", arg2.emit(options)).unwrap(),
        }
    }

    if let Some(line_comment) = comment
    {
        let the_comment = color_comment(
            format!("  ;; {}", line_comment),
            options
        );

        write!(writer, "{}", the_comment).unwrap();
    }

    writeln!(writer, "").unwrap();
}
