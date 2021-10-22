
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

impl std::fmt::Display for Operand
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self
        {
            Self::GeneralPurpose { register_index: index, indirect: at } =>
            {
                assert!((0u8 ..= 7u8).contains(&index));

                // TODO(pbz): Is it inconsistent to not have a space here?
                // TODO(pbz): Other natural index arguments have a space.

                write!(
                    f,
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
                    write!(f, "{}FLAGS", if *at { "@" } else { "" })
                }

                else
                {
                    write!(f, "{}IP", if *at { "@" } else { "" })
                }
            }
        }
    }
}
