use x86_64::{instructions::interrupts, structures::idt::InterruptStackFrame};

use crate::{terminal::input::TerminalInput, utils::time::get_mtl_datetime};

use super::setup::{InterruptIndex, PICS};

pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let time = get_mtl_datetime().time();
    interrupts::without_interrupts(|| {
        TerminalInput::global().set_label(format!("[{time}]$ ").as_str());
    });

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    };
}
