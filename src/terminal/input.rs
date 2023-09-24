use crate::{commands, eprintln, print, println};

use alloc::string::String;
use spin::{Mutex, Once};
use x86_64::instructions::port::Port;

unsafe fn outb(port: u16, value: u16) {
    let mut port = Port::new(port);
    port.write(value);
}

use super::{
    screen_char::{ScreenChar, SPACE_SCREEN_CHAR},
    vga_text::{VGAText, BUFFER_WIDTH},
};

pub struct TerminalInput {
    pub x: usize,
}

impl VGAText for TerminalInput {
    const RANGE: core::ops::Range<usize> = 24..25;
}

const LABEL_SIZE: usize = 12;

static INPUT: Once<Mutex<TerminalInput>> = Once::new();

impl TerminalInput {
    pub fn global() -> spin::MutexGuard<'static, TerminalInput> {
        INPUT.call_once(|| Mutex::new(TerminalInput::new())).lock()
    }

    pub fn new() -> Self {
        TerminalInput { x: 0 }
    }

    fn setx(&self, x: usize, char: ScreenChar) {
        self.set(x + LABEL_SIZE, Self::START, char);
    }

    fn getx(&self, x: usize) -> ScreenChar {
        self.get(x, Self::START)
    }

    fn clear(&self) {
        for col in LABEL_SIZE..BUFFER_WIDTH {
            self.set(col, Self::START, SPACE_SCREEN_CHAR);
        }
    }

    fn get_input(&self) -> String {
        let mut input = String::new();
        for col in LABEL_SIZE..BUFFER_WIDTH {
            input.push(self.getx(col).byte as char);
        }
        input
    }

    fn get_label(&self) -> String {
        let mut input = String::new();
        for col in 0..LABEL_SIZE {
            input.push(self.getx(col).byte as char);
        }
        input
    }

    pub fn set_label(&self, label: &str) {
        if label.len() == LABEL_SIZE {
            for (i, c) in label.char_indices() {
                self.set(i, Self::START, ScreenChar::highlighted(c as u8));
            }
        }
    }

    fn update_caret(&self) {
        unsafe {
            let pos = (24 * 80 + self.x + LABEL_SIZE) as u16;
            outb(0x3D4, 0x0F);
            outb(0x3D5, pos & 0xFF);

            outb(0x3D4, 0x0E);
            outb(0x3D5, (pos >> 8) & 0xFF);
        }
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

                self.setx(self.x, ScreenChar::highlighted(byte));
                self.x += 1;
            }
            // New Line (Enter Key)
            b'\n' => {
                print!("{}", self.get_label());
                let input = self.get_input();
                self.clear();
                println!("{}", input);
                // Triming is necessary because most of the input is 0s
                commands::handle_input(input.trim());
                self.x = 0;
            }
            // Unimplemented
            _ => eprintln!("Input: Unimplemented."),
        }

        self.update_caret();
    }
}
