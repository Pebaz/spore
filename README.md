
# Spore - UEFI Bytecode Disassembler

<!-- <p align=center>
    <img src="misc/Spore.png" width=50%>
</p> -->


<table>
    <tr>
        <td>
            <img src="misc/Spore.png" width=100%>
        </td>
        <td>
            <img src="misc/Disassembly1.png" width=100%>
        </td>
    </tr>
</table>

> A disassembler for the UEFI Bytecode Virtual Machine.


## Demo

<table>
<tr>
<td>

Given the following FASMG-EBC UEFI Bytecode Assembly file:

```x86asm
;; Adapted from https://github.com/pbatard/fasmg-ebc/blob/master/hello.asm

include 'ebc.inc'
include 'efi.inc'
include 'format.inc'
include 'utf8.inc'

format peebc efi  ;; PE executable format, EFI Byte Code

entry efi_main

section '.text' code executable readable

efi_main:
    MOVn   R1, @R0(EFI_MAIN_PARAMETERS.SystemTable)
    MOVn   R1, @R1(EFI_SYSTEM_TABLE.ConOut)
    MOVREL R2, string_hello
    PUSHn  R2
    PUSHn  R1
    CALLEX @R1(SIMPLE_TEXT_OUTPUT_INTERFACE.OutputString)
    MOV R0, R0(+2,0)
    JMP efi_main
    RET

section '.data' data readable writeable
    string_hello: du "Hello World!", 0x0A, 0x0
```
</td>

<td>

Compile it using [FASMG-EBC](https://github.com/pbatard/fasmg-ebc) by cloning
the project and putting the file into the project root.

Save it to a familiar name such as "bc.asm" for example.

```bash
# Generates `bc.efi`
$ make bc.asm

# This is a PE executable that contains UEFI Bytecode
$ file bc.efi
bc.efi: PE32+ executable (DLL) (EFI application) EFI byte code, for MS Windows
```
</td>
</tr>
<tr>
<td>
Now that we have a bootable PE executable, we can output the bytecode
instructions inside of it:

```bash
$ spore bc.efi
```
</td>
<td>
The disassembled bytecode instructions are then emitted by Spore:

```x86asm
      72 81 41 10  MOVnw R1, @R0(+1, +16)
      72 91 85 21  MOVnw R1, @R1(+5, +24)
      79 02 F4 0F  MOVRELw R2, 4084
            35 02  PUSHn R2
            35 01  PUSHn R1
83 29 01 00 00 10  CALL32EXa @R1(+1, +0)
      60 00 02 10  MOVqw R0, R0(+2, +0)
            02 F2  JMP8 -14
               04  RET
```
</td>
<td>

</td>
</tr>
</table>

## Installation

```bash
$ cargo install spore

# Or alternatively
$ cargo install --git https://github.com/Pebaz/spore
```

## Why

<!-- <table>
    <tr>
        <td>
            Assembly written using FASMG-EBC UEFI Bytecode Assembler
            <img src="misc/Assembly1.png">
            <img src="misc/Disassembly1.png">
        </td>
        <td>
            Output
            <img src="misc/Disassembly1.png">
            <img src="misc/Disassembly1.png">
        </td>
    </tr>
</table> -->

## Usage

<p align=center>
    <img src="misc/Usage.png" width=75%>
</p>

## Notes

* Thank you to [Pete Batard](https://github.com/pbatard) for creating
  [FASMG-EBC](https://github.com/pbatard/fasmg-ebc) which is based on the Flat
  Assembler. Without this tool, I would not have had the assembly files to
  disassemble!
* The [UEFI Specification](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf)
  is exceptionally well-written and contained all the information necessary to
  implement Spore.
* Although Spore is cross-platform (Windows, MacOS, Linux), I have not tested
  whether FASMG-EBC works on other platforms.
