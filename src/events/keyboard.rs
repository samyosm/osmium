use pc_keyboard::DecodedKey;
use x86_64::instructions::interrupts;

use crate::terminal::input::TerminalInput;

type KeyboardHandler = fn(DecodedKey) -> ();

pub static KEYBOARD_EVENT_LISTENERS: &[KeyboardHandler] = &[input_handler];

pub fn keyboard_event_handler(key: DecodedKey) {
    for handler in KEYBOARD_EVENT_LISTENERS.iter() {
        handler(key);
    }
}

fn input_handler(key: DecodedKey) {
    if let DecodedKey::Unicode(char) = key {
        interrupts::without_interrupts(|| {
            TerminalInput::global().input_char(char as u8);
        })
    }
}
