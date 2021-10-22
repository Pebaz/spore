use crate::theme::Theme;

pub struct Options
{
    pub theme: Option<Theme>,  // Colorize the assembly output (optional for pipes)

    // Show bytecode bytes on the left of the assembly output
    // bytecode: bool,
}
