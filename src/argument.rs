use crate::natural_index::NaturalIndex;
use crate::options::Options;
use crate::theme::*;

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

impl Emit for Argument
{
    fn emit(&self, options: &Options) -> String
    {
        match self
        {
            Self::Index16(index) =>
            {
                let natural_index = NaturalIndex::from_u16(*index);

                format!("{}", natural_index.emit(options))
            }

            Self::Index32(index) =>
            {
                let natural_index = NaturalIndex::from_u32(*index);

                format!("{}", natural_index.emit(options))
            }

            Self::Index64(index) =>
            {
                let natural_index = NaturalIndex::from_u64(*index);

                format!("{}", natural_index.emit(options))
            }

            Self::ImmediateU16(immediate) =>
            {
                color_immediate(immediate.to_string(), options)
            }

            Self::ImmediateU32(immediate) =>
            {
                color_immediate(immediate.to_string(), options)
            }

            // Self::ImmediateU64(immediate) =>
            // {
            //     color_immediate(immediate.to_string(), options)
            // }

            Self::ImmediateI16(immediate) =>
            {
                color_immediate(immediate.to_string(), options)
            }

            Self::ImmediateI32(immediate) =>
            {
                color_immediate(immediate.to_string(), options)
            }

            Self::ImmediateI64(immediate) =>
            {
                color_immediate(immediate.to_string(), options)
            }
        }
    }
}
