use colored::*;
use crate::options::Options;

pub const SPORE_THEME: Theme = Theme
{
    opcode: color(98, 168, 209),
    error: color(255, 0, 55),
    bytecode: color(55, 55, 55),
    indirect: color(255, 0, 255),
    operand: color(255, 0, 255),
    index: color(255, 0, 255),
    immediate: color(255, 0, 255),
    comment: color(255, 0, 255),
    x8: color(255, 0, 255),
    x16: color(255, 0, 255),
    x32: color(255, 0, 255),
    x64: color(255, 0, 255),
};

// TODO(pbz): Gravity Falls McGucket theme like that old computer!
// TODO(pbz): Matterhorn Village Theme

pub struct Theme
{
    // (Foreground, Background, Style)
    pub opcode: Color,  // (Option<Color>, Option<Color>, Option<Style>),
    pub error: Color,
    pub bytecode: Color,
    pub indirect: Color,
    pub operand: Color,
    pub index: Color,
    pub immediate: Color,
    pub comment: Color,
    pub x8: Color,
    pub x16: Color,
    pub x32: Color,
    pub x64: Color,
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

    else { string }
}

// TODO(pbz): Perhaps no more need for this once Options are passed directly to
// TODO(pbz): .color() methods on each type.
pub fn color_opcode(string: String, options: &Options) -> String
{
    if let Some(color_theme) = &options.theme
    {
        colored_string(string, color_theme.opcode)
    }

    else { string }
}

pub fn color_error(string: String, options: &Options) -> String
{
    if let Some(color_theme) = &options.theme
    {
        colored_string(string, color_theme.error)
    }

    else { string }
}

pub fn color_bytecode(string: String, options: &Options) -> String
{
    if let Some(color_theme) = &options.theme
    {
        colored_string(string, color_theme.bytecode)
    }

    else { string }
}

pub fn color_indirect(string: String, options: &Options) -> String
{
    if let Some(color_theme) = &options.theme
    {
        colored_string(string, color_theme.indirect)
    }

    else { string }
}

pub fn color_operand(string: String, options: &Options) -> String
{
    if let Some(color_theme) = &options.theme
    {
        colored_string(string, color_theme.operand)
    }

    else { string }
}

pub fn color_index(string: String, options: &Options) -> String
{
    if let Some(color_theme) = &options.theme
    {
        colored_string(string, color_theme.index)
    }

    else { string }
}

pub fn color_immediate(string: String, options: &Options) -> String
{
    if let Some(color_theme) = &options.theme
    {
        colored_string(string, color_theme.immediate)
    }

    else { string }
}

pub fn color_comment(string: String, options: &Options) -> String
{
    if let Some(color_theme) = &options.theme
    {
        colored_string(string, color_theme.comment)
    }

    else { string }
}


pub fn color_x8(string: String, options: &Options) -> String
{
    if let Some(color_theme) = &options.theme
    {
        colored_string(string, color_theme.x8)
    }

    else { string }
}

pub fn color_x16(string: String, options: &Options) -> String
{
    if let Some(color_theme) = &options.theme
    {
        colored_string(string, color_theme.x16)
    }

    else { string }
}

pub fn color_x32(string: String, options: &Options) -> String
{
    if let Some(color_theme) = &options.theme
    {
        colored_string(string, color_theme.x32)
    }

    else { string }
}

pub fn color_x64(string: String, options: &Options) -> String
{
    if let Some(color_theme) = &options.theme
    {
        colored_string(string, color_theme.x64)
    }

    else { string }
}


pub trait Emit
{
    // TODO(pbz): emit(string_buffer: &mut ArrayString, ...)
    fn emit(&self, options: &Options) -> String;
}
