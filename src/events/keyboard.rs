use alloc::vec::Vec;
use lazy_static::lazy_static;
use spin::Mutex;

use crate::utils::write_macros::INPUT;

type KeyboardHandler = fn(char) -> ();

lazy_static! {
    pub static ref KEYBOARD_EVENT_LISTENERS: Mutex<Vec<KeyboardHandler>> = Mutex::new(Vec::new());
}

pub fn keyboard_event_handler(char: char) {
    INPUT.lock().input_char(char as u8);
    for handler in KEYBOARD_EVENT_LISTENERS.lock().iter() {
        handler(char);
    }
}

pub fn add_keyboard_listener(handler: KeyboardHandler) {
    KEYBOARD_EVENT_LISTENERS.lock().push(handler);
}
