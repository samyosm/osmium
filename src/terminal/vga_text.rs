use core::ops::Range;

use lazy_static::lazy_static;
use spin::Mutex;

use super::screen_char::ScreenChar;

type Buffer = &'static mut [[ScreenChar; 80]; 25];
lazy_static! {
    pub static ref BUFFER: Mutex<Buffer> =
        Mutex::new(unsafe { &mut *(0xb8000 as *mut [[ScreenChar; 80]; 25]) });
}

pub const BUFFER_WIDTH: usize = 80;

pub trait VGAText {
    const RANGE: Range<usize>;
    const END: usize = Self::RANGE.end;
    const START: usize = Self::RANGE.start;

    fn set(&self, x: usize, y: usize, char: ScreenChar) {
        let buffer = &mut BUFFER.lock()[Self::RANGE];
        buffer[y - Self::START][x] = char;
    }

    fn get(&self, x: usize, y: usize) -> ScreenChar {
        BUFFER.lock()[y][x]
    }
}
