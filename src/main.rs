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

mod opcode;
mod operand;
mod argument;
mod natural_index;
mod instruction;
mod bits;

#[cfg(test)]
mod tests;  // Integration tests

use std::io;
use std::io::prelude::*;
use crate::opcode::OpCode;

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
            if OpCode::disassemble(&mut io::stdout(), &mut bytes).is_none()
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
