use core::fmt;

use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::interrupts;

use crate::terminal::{
    input::TerminalInput,
    output::{Output, TerminalOutput},
};

/* Singleton */
// TODO: Stop hard coding
lazy_static! {
    pub static ref INPUT: Mutex<TerminalInput> = Mutex::new(TerminalInput::new());
    pub static ref OUTPUT: Mutex<TerminalOutput> = Mutex::new(TerminalOutput::new());
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments, output: Output, new_line: bool) {
    interrupts::without_interrupts(|| {
        OUTPUT.lock().print(args, output);
        if new_line {
            OUTPUT.lock().new_line();
        }
    })
}

/* Macros */
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        $crate::utils::write_macros::_print(format_args!($($arg)*), crate::terminal::output::Output::Std, true)
    }
}

#[macro_export]
macro_rules! eprintln {
    ($($arg:tt)*) => {
        $crate::utils::write_macros::_print(format_args!($($arg)*), crate::terminal::output::Output::Err, true)
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::utils::write_macros::_print(format_args!($($arg)*), crate::terminal::output::Output::Std, false)
    }
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => {
        $crate::utils::write_macros::_print(format_args!($($arg)*), crate::terminal::output::Output::Err, false)
    }
}
