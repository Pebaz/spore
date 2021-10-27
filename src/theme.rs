use colored::*;
use crate::options::Options;

pub const SPORE_THEME: Theme = Theme
{
    opcode: color(98, 168, 209),
    error: color(255, 0, 55),
};

// TODO(pbz): Gravity Falls McGucket theme like that old computer!
// TODO(pbz): Matterhorn Village Theme

pub struct Theme
{
    // (Foreground, Background, Style)
    pub opcode: Color,  // (Option<Color>, Option<Color>, Option<Style>),
    pub error: Color,
    // pub bytecode: Color,
    // pub operand: Color,
    // pub index: Color,
    // pub immediate: Color,
    // pub comment: Color,
}

pub const fn color(r: u8, g: u8, b: u8) -> Color
{
    Color::TrueColor { r, g, b }
}

pub fn colored_string(string: String, color: Color) -> String
{
    if let Color::TrueColor { r, g, b } = color
    {
        string.truecolor(r, g, b).to_string()
    }
    else
    {
        string
    }
}


// TODO(pbz): Perhaps no more need for this once Options are passed directly to
// TODO(pbz): .color() methods on each type.
pub fn color_opcode(string: String, options: &Options) -> String
{
    if let Some(color_theme) = &options.theme
    {
        colored_string(string, color_theme.opcode)
    }
    else
    {
        string
    }
}

pub fn color_error(string: String, options: &Options) -> String
{
    if let Some(color_theme) = &options.theme
    {
        colored_string(string, color_theme.error)
    }
    else
    {
        string
    }
}

pub trait Emit
{
    // TODO(pbz): emit(string_buffer: &mut ArrayString, ...)
    fn emit(&self, options: &Options) -> String;
}
