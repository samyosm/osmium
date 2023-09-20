use core::fmt::{self, Arguments, Write};
use core::mem::MaybeUninit;

use spin::Once;

use crate::color::{Color, ColorCode};

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

pub unsafe fn write_char(x: usize, y: usize, byte: u8, code: ColorCode) {
    // 160 is the number of rows in VGA mode * 2 (because each row is 2 byte long)
    let vga_buffer = (0xb8000 + x * 2 + y * BUFFER_WIDTH * 2) as *mut u8;
    unsafe {
        *vga_buffer = byte;
        *vga_buffer.offset(1) = code.0;
    };
}

pub struct Writer {
    x: usize,
    y: usize,
    color: ColorCode,
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print(s);
        Ok(())
    }
}

// TODO: Support fmt
impl Writer {
    pub fn new() -> Self {
        Writer {
            x: 0,
            y: 0,
            color: ColorCode::default(),
        }
    }

    fn set_print_mode(&mut self) {
        self.color = ColorCode::default();
    }

    fn new_line(&mut self) {
        self.x = 0;
        self.y += 1;
    }

    fn print_char(&mut self, byte: u8) {
        if self.y > BUFFER_HEIGHT {
            panic!("Can't print more than 25 rows.")
        }

        if self.x > BUFFER_WIDTH || byte == b'\n' {
            self.new_line();
        }

        match byte {
            // printable ASCII byte
            0x20..=0x7e => {
                unsafe { write_char(self.x, self.y, byte, self.color) };
                self.x += 1;
            }
            // not part of printable ASCII range
            _ => {}
        }
    }

    pub fn print(&mut self, text: &str) {
        for &byte in text.as_bytes() {
            self.print_char(byte);
        }
    }

    pub fn print_fmt(&mut self, arguments: Arguments) {
        let _ = self.write_fmt(arguments);
    }

    pub fn println(&mut self, arguments: Arguments) {
        self.print_fmt(arguments);
        self.new_line();
    }

    pub fn eprint(&mut self, arguments: Arguments) {
        self.clear();

        let error_code: ColorCode = ColorCode::new(Color::Red, Color::Black);
        self.color = error_code;
        self.print_fmt(arguments);
        self.set_print_mode();
    }

    pub fn eprintln(&mut self, arguments: Arguments) {
        self.eprint(arguments);
        self.new_line();
    }

    pub fn clear(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                unsafe { write_char(col, row, b' ', ColorCode::default()) }
            }
        }
        self.x = 0;
        self.y = 0;
    }
}

/** Singleton */
static mut OUTPUT: Once<MaybeUninit<Writer>> = Once::new();

pub fn get_output() -> &'static mut Writer {
    unsafe {
        // This is safe because the only way to get OUTPUT is to call this method.
        // And this method does initialize OUTPUT.
        OUTPUT.call_once(|| MaybeUninit::new(Writer::new()));
        // This is safe because we've initialized it just above.
        OUTPUT.get_mut_unchecked().assume_init_mut()
    }
}

/* Macro */

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        write::get_output().println(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! eprintln {
    ($($arg:tt)*) => {
        write::get_output().eprintln(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        write::get_output().print_fmt(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => {
        write::get_output().eprint(format_args!($($arg)*));
    };
}
