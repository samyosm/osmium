use alloc::vec::Vec;
use lazy_static::lazy_static;
use pc_keyboard::DecodedKey;
use spin::Mutex;
use x86_64::instructions::interrupts;

use crate::terminal::input::TerminalInput;

type KeyboardHandler = fn(DecodedKey) -> ();

lazy_static! {
    pub static ref KEYBOARD_EVENT_LISTENERS: Mutex<Vec<KeyboardHandler>> =
        Mutex::new(vec![input_handler]);
}

pub fn keyboard_event_handler(key: DecodedKey) {
    for handler in KEYBOARD_EVENT_LISTENERS.lock().iter() {
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

pub fn add_keyboard_listener(handler: KeyboardHandler) {
    KEYBOARD_EVENT_LISTENERS.lock().push(handler);
}
