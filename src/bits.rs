
// pub fn bits(byte: u8) -> [bool; 8]
// {
//     let mut bits = [false; 8];

//     for i in 0 .. 8
//     {
//         if byte & 2u8.pow(i) > 0
//         {
//             bits[(bits.len() - 1) - i as usize] = true;
//         }
//     }

//     bits
// }


// pub fn bits_u8(byte: u8) -> [bool; 8]
// {
//     let mut bits = [false; 8];

//     for i in 0 .. 8
//     {
//         if byte & 2u8.pow(i) > 0
//         {
//             bits[(bits.len() - 1) - i as usize] = true;
//         }
//     }

//     bits
// }

// pub fn bits_to_byte_u8(bits: &[bool]) -> u8
// {
//     let mut byte = 0;

//     for (i, bit) in bits.iter().rev().enumerate()
//     {
//         if *bit
//         {
//             // byte += 2u8.pow((bits.len() - 1 - i) as u32);
//             byte += 2u8.pow((i) as u32);
//         }
//     }
//     byte
// }

pub fn bits_u16(byte: u16) -> [bool; 16]
{
    let mut bits = [false; 16];

    for i in 0 .. 16
    {
        if byte & 2u16.pow(i) > 0
        {
            bits[(bits.len() - 1) - i as usize] = true;
        }
    }

    bits
}

pub fn bits_to_byte_u16(bits: &[bool]) -> u16
{
    let mut byte = 0;

    for (i, bit) in bits.iter().rev().enumerate()
    {
        if *bit
        {
            byte += 2u16.pow((i) as u32);
        }
    }
    byte
}

pub fn bits_u32(byte: u32) -> [bool; 32]
{
    let mut bits = [false; 32];

    for i in 0 .. 32
    {
        if byte & 2u32.pow(i) > 0
        {
            bits[(bits.len() - 1) - i as usize] = true;
        }
    }

    bits
}

pub fn bits_to_byte_u32(bits: &[bool]) -> u32
{
    let mut byte = 0;

    for (i, bit) in bits.iter().rev().enumerate()
    {
        if *bit
        {
            byte += 2u32.pow((i) as u32);
        }
    }
    byte
}

pub fn bits_u64(byte: u64) -> [bool; 64]
{
    let mut bits = [false; 64];

    for i in 0 .. 64
    {
        if byte & 2u64.pow(i) > 0
        {
            bits[(bits.len() - 1) - i as usize] = true;
        }
    }

    bits
}

pub fn bits_to_byte_u64(bits: &[bool]) -> u64
{
    let mut byte = 0;

    for (i, bit) in bits.iter().rev().enumerate()
    {
        if *bit
        {
            byte += 2u64.pow((i) as u32);
        }
    }
    byte
}

/// Returns the bits of a byte in reverse so that indexing works as expected.
pub fn bits_rev(byte: u8) -> [bool; 8]
{
    let mut bits = [false; 8];

    for i in 0 .. 8
    {
        if byte & 2u8.pow(i) > 0
        {
            bits[i as usize] = true;
        }
    }

    bits
}

/// Converts a slice of bits sorted in reverse to a byte.
pub fn bits_to_byte_rev(bits: &[bool]) -> u8
{
    let mut byte = 0;

    for (i, bit) in bits.iter().enumerate()
    {
        if *bit
        {
            byte += 2u8.pow((i) as u32);
        }
    }
    byte
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    pub fn test_bits_to_byte()
    {
        assert_eq!(bits_to_byte_u8(&[true, false]), 2u8);
        assert_eq!(bits_to_byte_u8(&[false, true, false]), 2u8);
        assert_eq!(bits_to_byte_u8(&[true, false, false]), 4u8);
        assert_eq!(bits_to_byte_u8(&[true, false, false, false]), 8u8);
        assert_eq!(bits_to_byte_u8(&[true, false, false, true]), 9u8);
        assert_eq!(bits_to_byte_u8(&[true, false, true, true]), 11u8);

        assert_eq!(bits_to_byte_u32(&[true, false]), 2u32);
        assert_eq!(bits_to_byte_u32(&[false, true, false]), 2u32);
        assert_eq!(bits_to_byte_u32(&[true, false, false]), 4u32);
        assert_eq!(bits_to_byte_u32(&[true, false, false, false]), 8u32);
        assert_eq!(bits_to_byte_u32(&[true, false, false, true]), 9u32);
        assert_eq!(bits_to_byte_u32(&[true, false, true, true]), 11u32);

        assert_eq!(bits_to_byte_u64(&[true, false]), 2u64);
        assert_eq!(bits_to_byte_u64(&[false, true, false]), 2u64);
        assert_eq!(bits_to_byte_u64(&[true, false, false]), 4u64);
        assert_eq!(bits_to_byte_u64(&[true, false, false, false]), 8u64);
        assert_eq!(bits_to_byte_u64(&[true, false, false, true]), 9u64);
        assert_eq!(bits_to_byte_u64(&[true, false, true, true]), 11u64);
    }

    #[test]
    pub fn test_bits()
    {
        assert_eq!(
            bits_u8(2u8),
            [false, false, false, false, false, false, true, false]
        );

        assert_eq!(
            bits_u8(4u8),
            [false, false, false, false, false, true, false, false]
        );

        assert_eq!(
            bits_u8(0x32u8),
            [false, false, true, true, false, false, true, false]
        );
    }

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
