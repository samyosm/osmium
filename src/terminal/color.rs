pub const DEFAULT_COLOR: ColorCode = ColorCode::with_black_bg(Color::LightGreen);
pub const ERROR_COLOR: ColorCode = ColorCode::with_black_bg(Color::LightRed);
pub const NUMBER_COLOR: ColorCode = ColorCode::with_black_bg(Color::LightCyan);
pub const HIGHLIGHT_COLOR: ColorCode = ColorCode::with_black_bg(Color::Yellow);

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(pub u8);

impl ColorCode {
    pub const fn new(foreground: Color, background: Color) -> Self {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }

    pub const fn with_black_bg(foreground: Color) -> Self {
        ColorCode::new(foreground, Color::Black)
    }
}

impl Default for ColorCode {
    fn default() -> Self {
        ColorCode::new(Color::LightGreen, Color::Black)
    }
}
