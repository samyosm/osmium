use super::color::{ColorCode, DEFAULT_COLOR};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    pub byte: u8,
    pub color_code: ColorCode,
}

pub const SPACE_SCREEN_CHAR: ScreenChar = ScreenChar {
    byte: b' ',
    color_code: DEFAULT_COLOR,
};
