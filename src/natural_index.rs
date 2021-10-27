use crate::options::Options;
use crate::theme::*;
use crate::bits::*;

const SIZE_OF_VOID_PTR: u16 = 8;
const HEADER_SIZE: usize = 4;

pub struct NaturalIndex
{
    pub value: u64,
    pub sign: i8,
    pub constant: u64,
    pub natural: u64,
    pub offset: i64,
}

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

impl Emit for NaturalIndex
{
    fn emit(&self, options: &Options) -> String
    {
        format!(
            "({}{}, {}{})",
            if self.sign < 0 { "-" } else { "+" },
            color_index(self.natural.to_string(), options),
            if self.sign < 0 { "-" } else { "+" },
            color_index(self.constant.to_string(), options)
        )
    }
}


#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    pub fn test_natural_indexing()
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
