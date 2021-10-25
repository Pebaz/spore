use crate::natural_index::*;

pub enum Argument
{
    Index16(u16),
    Index32(u32),
    Index64(u64),
    ImmediateU16(u16),
    ImmediateU32(u32),
    // ImmediateU64(u64),  // Commenting out for now since nothing is using it
    ImmediateI16(i16),
    ImmediateI32(i32),
    ImmediateI64(i64),
}

impl std::fmt::Display for Argument
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self
        {
            Self::Index16(index) =>
            {
                let natural_index = NaturalIndex::from_u16(*index);

                write!(f, "{}", natural_index)
            }

            Self::Index32(index) =>
            {
                let natural_index = NaturalIndex::from_u32(*index);

                write!(f, "{}", natural_index)
            }

            Self::Index64(index) =>
            {
                let natural_index = NaturalIndex::from_u64(*index);

                write!(f, "{}", natural_index)
            }

            Self::ImmediateU16(immediate) => write!(f, "{}", immediate),

            Self::ImmediateU32(immediate) => write!(f, "{}", immediate),

            // Self::ImmediateU64(immediate) => write!(f, "{}", immediate),

            Self::ImmediateI16(immediate) => write!(f, "{}", immediate),

            Self::ImmediateI32(immediate) => write!(f, "{}", immediate),

            Self::ImmediateI64(immediate) => write!(f, "{}", immediate),
        }
    }
}
