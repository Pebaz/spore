use crate::instruction::*;
use crate::options::Options;
use crate::bits::*;
use crate::theme::Emit;


#[derive(Debug)]
pub enum OpCode
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

impl std::convert::TryFrom<OpCode> for u8
{
    type Error = ();

    fn try_from(v: OpCode) -> Result<Self, Self::Error>
    {
        match v
        {
            OpCode::ADD => Ok(0x0C),
            OpCode::AND => Ok(0x14),
            OpCode::ASHR => Ok(0x19),
            OpCode::BREAK => Ok(0x00),
            OpCode::CALL => Ok(0x03),
            OpCode::CMPeq => Ok(0x05),
            OpCode::CMPlte => Ok(0x06),
            OpCode::CMPgte => Ok(0x07),
            OpCode::CMPulte => Ok(0x08),
            OpCode::CMPugte => Ok(0x09),
            OpCode::CMPIeq => Ok(0x2D),
            OpCode::CMPIlte => Ok(0x2E),
            OpCode::CMPIgte => Ok(0x2F),
            OpCode::CMPIulte => Ok(0x30),
            OpCode::CMPIugte => Ok(0x31),
            OpCode::DIV => Ok(0x10),
            OpCode::DIVU => Ok(0x11),
            OpCode::EXTNDB => Ok(0x1A),
            OpCode::EXTNDD => Ok(0x1C),
            OpCode::EXTNDW => Ok(0x1B),
            OpCode::JMP => Ok(0x01),
            OpCode::JMP8 => Ok(0x02),
            OpCode::LOADSP => Ok(0x29),
            OpCode::MOD => Ok(0x12),
            OpCode::MODU => Ok(0x13),
            OpCode::MOVbw => Ok(0x1D),
            OpCode::MOVww => Ok(0x1E),
            OpCode::MOVdw => Ok(0x1F),
            OpCode::MOVqw => Ok(0x20),
            OpCode::MOVbd => Ok(0x21),
            OpCode::MOVwd => Ok(0x22),
            OpCode::MOVdd => Ok(0x23),
            OpCode::MOVqd => Ok(0x24),
            OpCode::MOVqq => Ok(0x28),
            OpCode::MOVI => Ok(0x37),
            OpCode::MOVIn => Ok(0x38),
            OpCode::MOVnw => Ok(0x32),
            OpCode::MOVnd => Ok(0x33),
            OpCode::MOVREL => Ok(0x39),
            OpCode::MOVsnw => Ok(0x25),
            OpCode::MOVsnd => Ok(0x26),
            OpCode::MUL => Ok(0x0E),
            OpCode::MULU => Ok(0x0F),
            OpCode::NEG => Ok(0x0B),
            OpCode::NOT => Ok(0x0A),
            OpCode::OR => Ok(0x15),
            OpCode::POP => Ok(0x2C),
            OpCode::POPn => Ok(0x36),
            OpCode::PUSH => Ok(0x2B),
            OpCode::PUSHn => Ok(0x35),
            OpCode::RET => Ok(0x04),
            OpCode::SHL => Ok(0x17),
            OpCode::SHR => Ok(0x18),
            OpCode::STORESP => Ok(0x2A),
            OpCode::SUB => Ok(0x0D),
            OpCode::XOR => Ok(0x16),
        }
    }
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

impl OpCode
{
    /// Bytes are read from left to right. Bits are read from right to left.
    pub fn disassemble<T: Iterator<Item=u8>, W: std::io::Write>(
        options: &Options,
        writer: &mut W,
        bytes: &mut T
    ) -> Result<(), String>
    {
        let byte0 = if let Some(byte) = bytes.next()
        {
            byte
        }
        else
        {
            return Err(String::from("Completed Disassembly"));
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
            OpCode::RET =>
            {
                parse_instruction1(
                    writer,
                    options,
                    bytes,
                    byte0,
                    byte0_bits,
                    op
                )
            }

            OpCode::JMP8
            | OpCode::BREAK =>
            {
                // 2. INSTRUCTION ARGUMENT (BREAK)
                parse_instruction2(
                    writer,
                    options,
                    bytes,
                    byte0,
                    byte0_bits,
                    op
                )
            }

            OpCode::CALL
            | OpCode::JMP
            | OpCode::PUSH
            | OpCode::PUSHn
            | OpCode::POP
            | OpCode::POPn =>
            {
                // 3. INSTRUCTION OP1 ARGUMENT (CALL)
                parse_instruction3(
                    writer,
                    options,
                    bytes,
                    byte0,
                    byte0_bits,
                    op
                )
            }

            OpCode::LOADSP
            | OpCode::STORESP =>
            {
                // 4. INSTRUCTION OP1, OP2 (STORESP)
                parse_instruction4(
                    writer,
                    options,
                    bytes,
                    byte0,
                    byte0_bits,
                    op
                )
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
                parse_instruction5(
                    writer,
                    options,
                    bytes,
                    byte0,
                    byte0_bits,
                    op
                )
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
                parse_instruction6(
                    writer,
                    options,
                    bytes,
                    byte0,
                    byte0_bits,
                    op
                )
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
                parse_instruction7(
                    writer,
                    options,
                    bytes,
                    byte0,
                    byte0_bits,
                    op
                )
            }
        }
    }

    pub fn to(self) -> u8
    {
        self.try_into().unwrap()
    }
}

impl Emit for OpCode
{
    fn emit(&self, options: &Options) -> String
    {
        format!("{:?}", self)
    }
}
