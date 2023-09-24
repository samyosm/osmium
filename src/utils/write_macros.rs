use core::fmt;

use x86_64::instructions::interrupts;

use crate::terminal::output::{Output, TerminalOutput};

/* Singleton */
#[doc(hidden)]
pub fn _print(args: fmt::Arguments, output: Output, new_line: bool) {
    interrupts::without_interrupts(|| {
        TerminalOutput::global().print(args, output);
        if new_line {
            TerminalOutput::global().new_line();
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
