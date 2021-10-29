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
use crate::opcode::OpCode;
use crate::options::Options;
use crate::theme::*;


use std::fs::File;
use std::path::Path;
use pelite::{FileMap, Result};
use pelite::pe64::{Pe, PeFile};

const CODE_SECTION: u32 = pelite::image::IMAGE_SCN_CNT_CODE;
const HELP: &'static str = include_str!("CLI.txt");

/// Reads in an EFI Bytecode filename from STDIN and prints the disassembly.
fn main()
{
    let mut args = std::env::args().skip(1).collect::<Vec<String>>();

    if args.len() == 0
    {
        return println!("{}", HELP);
    }

    println!("{:?}", args);
    let filename = args.pop();
    println!("{:?}", args);
    println!("{:?}", filename);
    // let args = args.join(" ");
    // println!("{:?}", args);

    if args.len() % 2 != 0
    {
        return println!("{}", HELP);
    }

    let mut options = Options
    {
        theme: Some(SPORE),
        bytecode: true,
        pe: true,

        pad_output: true,  // ? Have this as a CLI option?
    };

    for i in (0 .. args.len()).step_by(2)
    {
        let option = args[i].clone();
        let value = args[i + 1].clone();

        match option.as_str()
        {
            "theme:" =>
            {
                if !"SPORE INDUSTRIAL_COMPUTER MATTERHORN_ZERMATT_VILLAGE OFF"
                    .contains(&value)
                {
                    println!(
                        "{}",
                        color_error(
                            format!("Unknown theme: {}", value),
                            &options
                        ),
                    );
                    return println!("{}", HELP);
                }

                options.theme = match value.as_str()
                {
                    "SPORE" => Some(SPORE),
                    "INDUSTRIAL_COMPUTER" => Some(INDUSTRIAL_COMPUTER),
                    "MATTERHORN_ZERMATT_VILLAGE" =>
                    {
                        Some(MATTERHORN_ZERMATT_VILLAGE)
                    }
                    "OFF" => None,

                    _ => unreachable!(),
                };
            }

            "bytecode:" =>
            {
                if value != "ON".to_string() && value != "OFF".to_string()
                {
                    println!(
                        "{}",
                        color_error(
                            format!("Invalid bytecode setting: {}", value),
                            &options
                        ),
                    );
                    return println!(include_str!("CLI.txt"));
                }

                options.bytecode = value == "ON";
            }

            "pe:" =>
            {
                if value != "ON".to_string() && value != "OFF".to_string()
                {
                    println!(
                        "{}",
                        color_error(
                            format!("Invalid pe setting: {}", value),
                            &options
                        ),
                    );
                    return println!(include_str!("CLI.txt"));
                }

                options.pe = value == "ON";
            }


            _ =>
            {
                println!(
                    "{}",
                    color_error(
                        format!("Invalid setting: {}", option),
                        &options
                    ),
                );
                return println!(include_str!("CLI.txt"));
            }
        }
    }

    // let mut show_help = true;
    // let options = Options
    // {
    //     pad_output: true,
    //     theme: Some(SPORE),
    //     bytecode: true,
    // };

    // for bytecode_file in std::env::args().skip(1).take(1)
    // {
    //     show_help = false;

    //     let file = match std::fs::File::open(bytecode_file.clone())
    //     {
    //         Ok(file) => file,
    //         Err(msg) =>
    //         {
    //             return println!(
    //                 "{}",
    //                 color_error(format!("{}", msg), &options)
    //             );
    //         }
    //     };

    //     // let mut bytes = file.bytes().map(|b| b.unwrap());
    //     // let mut bytes = file.bytes().cloned();

    //     let mut bytecode_section = None;
    //     let file_map = FileMap::open("../glop/drive/EFI/BOOT/BOOTX64.efi");

    //     if let Err(msg) = file_map
    //     {
    //         return println!("{}", color_error(format!("{}", msg), &options));
    //     }

    //     let file_bytes = file_map.unwrap();
    //     let file = PeFile::from_bytes(&file_bytes).unwrap();

    //     // Find the section header for code
    //     for section_header in file.section_headers()
    //     {
    //         if section_header.Characteristics & CODE_SECTION != 0
    //         {
    //             bytecode_section = Some(
    //                 file.get_section_bytes(section_header).unwrap()
    //             );

    //             break;
    //         }
    //     }

    //     match bytecode_section
    //     {
    //         Some(code_bytes) =>
    //         {
    //             let mut bytes = code_bytes.iter().cloned();

    //             loop
    //             {
    //                 let result = OpCode::disassemble(
    //                     &options,
    //                     &mut io::stdout(),
    //                     &mut bytes
    //                 );

    //                 match result
    //                 {
    //                     Ok(_) => (),

    //                     Err(msg) =>
    //                     {
    //                         println!("{}", msg);
    //                         break;
    //                     }
    //                 }
    //             }
    //         }

    //         None =>
    //         {
    //             return println!(
    //                 "{}",
    //                 color_error(
    //                     "PE file did not contain bytecode section".to_string(),
    //                     &options
    //                 )
    //             );
    //         }
    //     }
    // }

    // if show_help
    // {
    //     println!(include_str!("CLI.txt"));
    // }
}
