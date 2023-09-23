use alloc::vec::Vec;
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptStackFrame;

use crate::{
    events,
    interrupts::setup::{InterruptIndex, PICS},
};

use spin::Mutex;

lazy_static! {
    pub static ref KEYBOARD_EVENT_LISTENERS: Mutex<Vec<fn(char) -> ()>> = Mutex::new(vec![]);
}

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use x86_64::instructions::port::Port;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(
                ScancodeSet1::new(),
                layouts::Us104Key,
                HandleControl::Ignore
            ));
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(char) => {
                    events::keyboard::keyboard_event_handler(char);
                }
                // Keys like LShift, RShift, Ctrl...
                DecodedKey::RawKey(_key) => {}
            }
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    };
}
