use crate::theme::Theme;

pub struct Options
{
    pub theme: Option<Theme>,  // Colorize assembly output (optional for pipes)
    pub bytecode: bool,  // Output bytecode in hex notation beside assembly

    // Show bytecode bytes on the left of the assembly output
    // bytecode: bool,
}
