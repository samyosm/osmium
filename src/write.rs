use core::fmt::{self, Arguments, Write};

use crate::color::{Color, ColorCode};

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const INPUT_HEIGHT: usize = 2;

const WRITE_LINE: usize = BUFFER_HEIGHT - INPUT_HEIGHT - 1;
const INPUT_LINE: usize = WRITE_LINE + INPUT_HEIGHT;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    byte: u8,
    color_code: ColorCode,
}

pub struct Terminal {
    x: usize,
    input_x: usize,
    buffer: &'static mut [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
    color: ColorCode,
}

impl fmt::Write for Terminal {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print_raw(s);
        Ok(())
    }
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            x: 0,
            input_x: 0,
            buffer: unsafe { &mut *(0xb8000 as *mut [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT]) },
            color: ColorCode::default(),
        }
    }

    fn write_char(&mut self, x: usize, y: usize, byte: u8) {
        self.buffer[y][x] = ScreenChar {
            byte,
            color_code: match byte {
                // " $ % ' ( ) * + , - . / : ; < = > ? @  [ \ ] ^ _  { | } ~
                0x22..=0x40 | 0x5b..=0x5f | 0x7b..=0x7e => {
                    ColorCode::new(Color::LightCyan, Color::Black)
                }
                _ => self.color,
            },
        };
    }

    fn clear_char(&mut self, x: usize, y: usize) {
        let blank = ScreenChar {
            byte: b' ',
            color_code: self.color,
        };

        self.buffer[y][x] = blank;
    }

    fn clear_row(&mut self, row: usize) {
        for col in 0..BUFFER_WIDTH {
            self.clear_char(col, row);
        }
    }

    pub fn input_char(&mut self, byte: u8) {
        match byte {
            // Backspace
            0x08 => {
                if self.input_x > 0 {
                    self.input_x -= 1;
                    self.clear_char(self.input_x, INPUT_LINE);
                }
            }
            // Printable ASCII byte
            0x20..=0x7e => {
                if self.input_x > BUFFER_WIDTH - 1 {
                    self.eprint(format_args!("Terminal: Reached input limit.\n"));
                    return;
                }

                self.write_char(self.input_x, INPUT_LINE, byte);
                self.input_x += 1;
            }
            // New Line (Enter Key)
            0x0a => {
                // TODO: Call a command instead of this debug thingy
                self.print_raw("Terminal: ");
                for col in 0..BUFFER_WIDTH {
                    self.print_char(self.buffer[INPUT_LINE][col].byte);
                }
                self.print_raw("\n");
                self.clear_row(INPUT_LINE);
                self.input_x = 0;
            }
            // Unimplemented
            _ => self.eprint(format_args!("Terminal: Unimplemented.\n")),
        }
    }

    fn print_char(&mut self, byte: u8) {
        if self.x > BUFFER_WIDTH - 1 || byte == b'\n' {
            self.new_line();
        }

        match byte {
            // printable ASCII byte
            0x20..=0x7e => {
                self.write_char(self.x, WRITE_LINE, byte);
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

    pub fn print(&mut self, arguments: Arguments) {
        let _ = self.write_fmt(arguments);
    }

    pub fn eprint(&mut self, arguments: Arguments) {
        let error_code: ColorCode = ColorCode::new(Color::LightRed, Color::Black);
        self.color = error_code;
        self.print(arguments);
        self.color = ColorCode::default();
    }

    pub fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT - 1 {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer[row][col];
                self.buffer[row - 1][col] = character;
            }
        }

        self.clear_row(WRITE_LINE);
        self.x = 0;
    }
}
