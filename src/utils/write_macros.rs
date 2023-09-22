use core::fmt;

use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::interrupts;

use crate::write::Terminal;

/* Singleton */
lazy_static! {
    pub static ref WRITER: Mutex<Terminal> = Mutex::new(Terminal::new());
}

#[doc(hidden)]
pub enum Output {
    Std,
    Err,
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments, output: Output, new_line: bool) {
    interrupts::without_interrupts(|| {
        match output {
            Output::Std => WRITER.lock().print(args),
            Output::Err => WRITER.lock().eprint(args),
        };
        if new_line {
            WRITER.lock().new_line();
        }
    })
}

/* Macros */
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        $crate::utils::write_macros::_print(format_args!($($arg)*), $crate::utils::write_macros::Output::Std, true)
    }
}

#[macro_export]
macro_rules! eprintln {
    ($($arg:tt)*) => {
        $crate::utils::write_macros::_print(format_args!($($arg)*), $crate::utils::write_macros::Output::Err, true)
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::utils::write_macros::_print(format_args!($($arg)*), $crate::write::Output::Std, false)
    }
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => {
        $crate::utils::write_macros::_print(format_args!($($arg)*), $crate::write::Output::Err, new_line: false)
    }
}
