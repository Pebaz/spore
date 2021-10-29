use crate::theme::Theme;

pub struct Options
{
    pub pad_output: bool,  // Padding is great for output but not for testing
    pub theme: Option<Theme>,  // Colorize assembly output (optional for pipes)
    pub bytecode: bool,  // Output bytecode in hex notation beside assembly
    pub pe: bool,  // Load a Windows PE file rather than a binary file
}
