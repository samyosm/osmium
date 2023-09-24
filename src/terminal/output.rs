use core::fmt::{self, Arguments, Write};

use spin::{Mutex, MutexGuard, Once};

use super::{
    color::ERROR_COLOR,
    screen_char::{ScreenChar, SPACE_SCREEN_CHAR},
    vga_text::{VGAText, BUFFER_WIDTH},
};

#[derive(Default)]
pub enum Output {
    #[default]
    Std,
    Err,
}

pub struct TerminalOutput {
    x: usize,
    output: Output,
}

impl fmt::Write for TerminalOutput {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print_raw(s);
        Ok(())
    }
}

impl VGAText for TerminalOutput {
    const RANGE: core::ops::Range<usize> = 0..24;
}

static OUTPUT: Once<Mutex<TerminalOutput>> = Once::new();

impl TerminalOutput {
    pub fn global() -> MutexGuard<'static, TerminalOutput> {
        OUTPUT
            .call_once(|| Mutex::new(TerminalOutput::new()))
            .lock()
    }
    pub fn new() -> Self {
        Self {
            x: 0,
            output: Output::Std,
        }
    }

    pub fn set_char(&mut self, x: usize, y: usize, byte: u8) {
        self.set(
            x,
            y,
            match self.output {
                Output::Std => ScreenChar::highlighted(byte),
                Output::Err => ScreenChar {
                    byte,
                    color_code: ERROR_COLOR,
                },
            },
        );
    }

    pub fn clear_char(&mut self, x: usize, y: usize) {
        self.set(x, y, SPACE_SCREEN_CHAR);
    }

    pub fn clear_row(&mut self, row: usize) {
        for col in 0..BUFFER_WIDTH {
            self.clear_char(col, row);
        }
    }

    pub fn print_char(&mut self, byte: u8) {
        if self.x > BUFFER_WIDTH - 1 || byte == b'\n' {
            self.new_line();
        }

        match byte {
            // printable ASCII byte
            0x20..=0x7e => {
                // -1 because that's the height of the input
                self.set_char(self.x, Self::END - 1, byte);
                self.x += 1;
            }
            // not part of printable ASCII range
            _ => {}
        };
    }

    pub fn print_raw(&mut self, text: &str) {
        for &byte in text.as_bytes() {
            self.print_char(byte);
        }
    }

    pub fn print(&mut self, arguments: Arguments, output: Output) {
        self.output = output;
        let _ = self.write_fmt(arguments);
        self.output = Output::default();
    }

    pub fn new_line(&mut self) {
        for row in Self::START + 1..Self::END {
            for col in 0..BUFFER_WIDTH {
                // Look at vga_text:26 if this causes problems.
                let character = self.get(col, row);
                self.set(col, row - 1, character);
            }
        }

        self.clear_row(Self::END - 1);
        self.x = 0;
    }
}
