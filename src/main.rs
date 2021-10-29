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

use pelite::FileMap;
use pelite::pe64::{Pe, PeFile};
use crate::opcode::OpCode;
use crate::options::Options;
use crate::theme::*;

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

    let filename = args.pop().unwrap();

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
                    return println!(
                        "{}",
                        color_error(
                            format!("Invalid bytecode setting: {}", value),
                            &options
                        ),
                    );
                }

                options.bytecode = value == "ON";
            }

            "pe:" =>
            {
                if value != "ON".to_string() && value != "OFF".to_string()
                {
                    return println!(
                        "{}",
                        color_error(
                            format!("Invalid pe setting: {}", value),
                            &options
                        ),
                    );
                }

                options.pe = value == "ON";
            }


            _ =>
            {
                return println!(
                    "{}",
                    color_error(
                        format!("Invalid setting: {}", option),
                        &options
                    ),
                );
            }
        }
    }

    match FileMap::open(filename.as_str())
    {
        Ok(file_bytes) =>
        {
            let byte_slice = if options.pe
            {
                let file = PeFile::from_bytes(&file_bytes).unwrap();
                let mut bytecode_section = None;

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
                    Some(bytecode) => bytecode,
                    None =>
                    {
                        return println!(
                            "{}",
                            color_error(
                                "PE file is missing code section".to_string(),
                                &options
                            )
                        );
                    }
                }
            }
            else
            {
                file_bytes.as_ref()
            };

            let mut bytes = byte_slice.iter().cloned();

            loop
            {
                let result = OpCode::disassemble(
                    &options,
                    &mut std::io::stdout(),
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

        Err(msg) =>
        {
            return println!("{}", color_error(format!("{}", msg), &options));
        }
    }
}
