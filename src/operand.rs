use crate::options::Options;
use crate::theme::Emit;

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
                assert!((0u8 ..= 7u8).contains(&index));

                format!(
                    "{}R{}",
                    if *at { "@" } else { "" },
                    index
                )
            }

            Self::Dedicated { register_index: index, indirect: at } =>
            {
                assert!((0u8 ..= 1u8).contains(&index));

                if *index == 0
                {
                    format!("{}FLAGS", if *at { "@" } else { "" })
                }

                else
                {
                    format!("{}IP", if *at { "@" } else { "" })
                }
            }
        }
    }
}
