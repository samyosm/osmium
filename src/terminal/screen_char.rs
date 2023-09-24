use super::color::{ColorCode, DEFAULT_COLOR, HIGHLIGHT_COLOR, NUMBER_COLOR};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    pub byte: u8,
    pub color_code: ColorCode,
}

impl ScreenChar {
    pub fn highlighted(byte: u8) -> Self {
        let color_code = match byte {
            b'0'..=b'9' => NUMBER_COLOR,
            b'[' | b']' | b'$' | b':' => HIGHLIGHT_COLOR,
            _ => DEFAULT_COLOR,
        };

        Self { byte, color_code }
    }
}

pub const SPACE_SCREEN_CHAR: ScreenChar = ScreenChar {
    byte: b' ',
    color_code: DEFAULT_COLOR,
};
