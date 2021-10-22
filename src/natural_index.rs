use crate::bits::*;

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
    pub fn from_u16(value: u16) -> Self
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
    pub fn from_u32(value: u32) -> Self
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
    pub fn from_u64(value: u64) -> Self
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
