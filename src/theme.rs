use colored::*;
use crate::options::Options;

pub const SPORE: Theme = Theme
{
    opcode: color(217, 207, 199),
    error: color(207, 66, 31),
    bytecode: color(84, 71, 52),
    indirect: color(227, 202, 113),
    operand: color(135, 75, 41),
    index: color(217, 182, 130),
    immediate: color(191, 151, 169),
    comment: color(135, 107, 55),
    x8: color(173, 152, 169),
    x16: color(221, 217, 159),
    x32: color(168, 132, 88),
    x64: color(125, 95, 102),
};

pub const INDUSTRIAL_COMPUTER: Theme = Theme
{
    opcode: color(161, 156, 148),
    error: color(255, 0, 0),
    bytecode: color(77, 75, 73),
    indirect: color(255, 153, 0),
    operand: color(161, 156, 148),
    index: color(255, 153, 0),
    immediate: color(255, 153, 0),
    comment: color(82, 214, 0),
    x8: color(0, 45, 122),
    x16: color(161, 78, 0),
    x32: color(3, 99, 0),
    x64: color(110, 18, 0),
};

pub const MATTERHORN_ZERMATT_VILLAGE: Theme = Theme
{
    opcode: color(173, 185, 201),
    error: color(255, 153, 0),
    bytecode: color(211, 195, 212),
    indirect: color(107, 129, 138),
    operand: color(115, 131, 153),
    index: color(140, 135, 128),
    immediate: color(199, 131, 64),
    comment: color(97, 93, 88),
    x8: color(110, 73, 35),
    x16: color(156, 103, 50),
    x32: color(199, 131, 64),
    x64: color(237, 156, 76),
};

pub struct Theme
{
    pub opcode: Color,
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
    fn emit(&self, options: &Options) -> String;
}
