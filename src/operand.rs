use crate::options::Options;
use crate::theme::*;

pub enum Operand
{
    GeneralPurpose
    {
        register_index: u8,
        indirect: bool,
    },

    Dedicated
    {
        register_index: u8,
        indirect: bool,
    },
}

impl Operand
{
    pub fn new_general_purpose(register_index: u8, indirect: bool) -> Self
    {
        assert!((0u8 ..= 7u8).contains(&register_index));

        Self::GeneralPurpose
        {
            register_index,
            indirect
        }
    }

    pub fn new_dedicated(register_index: u8, indirect: bool) -> Self
    {
        assert!((0u8 ..= 1u8).contains(&register_index));

        Self::Dedicated
        {
            register_index,
            indirect
        }
    }
}

impl Emit for Operand
{
    fn emit(&self, options: &Options) -> String
    {
        match self
        {
            Self::GeneralPurpose { register_index: index, indirect: at } =>
            {
                let at_sym = if *at
                {
                    color_indirect("@".to_string(), options)
                }
                else
                {
                    "".to_string()
                };

                assert!((0u8 ..= 7u8).contains(&index));

                color_operand(format!("{}R{}", at_sym, index), options)
            }

            Self::Dedicated { register_index: index, indirect: at } =>
            {
                let at_sym = if *at
                {
                    color_indirect("@".to_string(), options)
                }
                else
                {
                    "".to_string()
                };

                assert!((0u8 ..= 1u8).contains(&index));

                let result = if *index == 0
                {
                    format!("{}FLAGS", at_sym)
                }
                else
                {
                    format!("{}IP", at_sym)
                };

                color_operand(result, options)
            }
        }
    }
}
