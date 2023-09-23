use crate::{eprint, print, println, utils::write_macros::OUTPUT};

use x86_64::instructions::port::Port;

unsafe fn outb(port: u16, value: u16) {
    let mut port = Port::new(port);
    port.write(value);
}

unsafe fn inb(port: u16) -> u16 {
    Port::new(port).read()
}

use super::{
    color::DEFAULT_COLOR,
    screen_char::{ScreenChar, SPACE_SCREEN_CHAR},
    vga_text::{VGAText, BUFFER_WIDTH},
};

pub struct TerminalInput {
    pub x: usize,
}

impl VGAText for TerminalInput {
    const RANGE: core::ops::Range<usize> = 24..25;
}

impl TerminalInput {
    pub fn new() -> Self {
        TerminalInput { x: 0 }
    }

    fn setx(&self, x: usize, char: ScreenChar) {
        self.set(x, Self::START, char);
    }

    fn getx(&self, x: usize) -> ScreenChar {
        self.get(x, Self::START)
    }

    pub fn input_char(&mut self, byte: u8) {
        match byte {
            // Backspace
            0x08 => {
                if self.x > 0 {
                    self.x -= 1;
                    self.setx(self.x, SPACE_SCREEN_CHAR);
                }
            }
            // Printable ASCII byte
            0x20..=0x7e => {
                if self.x > BUFFER_WIDTH - 1 {
                    // self.terminal
                    //     .eprint(format_args!("Terminal: Reached input limit.\n"));
                    return;
                }

                self.setx(
                    self.x,
                    ScreenChar {
                        byte,
                        color_code: DEFAULT_COLOR,
                    },
                );
                self.x += 1;
            }
            // New Line (Enter Key)
            b'\n' => {
                // TODO: Call a command instead of this debug thingy
                print!("Terminal: ");
                for col in 0..BUFFER_WIDTH {
                    print!("{}", self.getx(col).byte as char);
                    self.setx(col, SPACE_SCREEN_CHAR);
                }
                print!("\n");
                self.x = 0;
            }
            // Unimplemented
            _ => eprint!("unimplemented."),
        }
        // TODO: Put into a function
        unsafe {
            let pos = 24 * 80 + self.x as u16;
            outb(0x3D4, 0x0F);
            outb(0x3D5, pos & 0xFF);

            outb(0x3D4, 0x0E);
            outb(0x3D5, (pos >> 8) & 0xFF);
        }
    }
}
