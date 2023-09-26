use core::ops::Range;

use spin::{Mutex, MutexGuard, Once};

use super::screen_char::ScreenChar;

static BUFFER: Once<Mutex<Buffer>> = Once::new();
struct Buffer(&'static mut [[ScreenChar; 80]; 25]);
impl Buffer {
    pub fn global() -> MutexGuard<'static, Buffer> {
        BUFFER
            .call_once(|| {
                Mutex::new(Buffer(unsafe {
                    &mut *(0xb8000 as *mut [[ScreenChar; 80]; 25])
                }))
            })
            .lock()
    }
}

pub const BUFFER_WIDTH: usize = 80;

pub trait VGAText {
    const RANGE: Range<usize>;
    const END: usize = Self::RANGE.end;
    const START: usize = Self::RANGE.start;

    fn set(&self, x: usize, y: usize, char: ScreenChar) {
        let buffer = &mut Buffer::global().0[Self::RANGE];
        buffer[y - Self::START][x] = char;
    }

    fn get(&self, x: usize, y: usize) -> ScreenChar {
        Buffer::global().0[y][x]
    }
}
