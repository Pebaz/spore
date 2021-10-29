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

mod opcode;
mod operand;
mod argument;
mod natural_index;
mod instruction;
mod bits;
mod options;
mod theme;

#[cfg(test)]
mod tests;  // Integration tests

use std::io;
use std::io::prelude::*;
use colored::*;
use crate::opcode::OpCode;
use crate::options::Options;
use crate::theme::*;


use std::fs::File;
use std::path::Path;
use pelite::{FileMap, Result};
use pelite::pe64::{Pe, PeFile};

const CODE_SECTION: u32 = pelite::image::IMAGE_SCN_CNT_CODE;

/// Reads in an EFI Bytecode filename from STDIN and prints the disassembly.
fn main()
{
    let mut show_help = true;
    let options = Options
    {
        pad_output: true,
        theme: Some(SPORE),
        bytecode: true,
    };

    for bytecode_file in std::env::args().skip(1).take(1)
    {
        show_help = false;

        let file = match std::fs::File::open(bytecode_file.clone())
        {
            Ok(file) => file,
            Err(msg) =>
            {
                println!("{}", color_error(format!("{}", msg), &options));
                return;
            }
        };

        // let mut bytes = file.bytes().map(|b| b.unwrap());
        // let mut bytes = file.bytes().cloned();

        let mut bytecode_section = None;
        let file_map = FileMap::open("../glop/drive/EFI/BOOT/BOOTX64.efi");

        if file_map.is_err()
        {
            println!("{}", "what?");
            return;
        }

        let file_bytes = file_map.unwrap();
        let file = PeFile::from_bytes(&file_bytes).unwrap();

        // Find the section header for code
        for section_header in file.section_headers()
        {
            if section_header.Characteristics & CODE_SECTION != 0
            {
                bytecode_section = Some(
                    file.get_section_bytes(section_header).unwrap()
                );

                break;
            }
        }

        match bytecode_section
        {
            Some(code_bytes) =>
            {
                let mut bytes = code_bytes.iter().cloned();

                loop
                {
                    let result = OpCode::disassemble(
                        &options,
                        &mut io::stdout(),
                        &mut bytes
                    );

                    match result
                    {
                        Ok(_) => (),

                        Err(msg) =>
                        {
                            println!("{}", msg);
                            break;
                        }
                    }
                }
            }

            None =>
            {
                println!("{}", "what?");
                return;
            }
        }
    }

    if show_help
    {
        println!(include_str!("CLI.txt"));
    }
}
