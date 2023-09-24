use alloc::{boxed::Box, string::String};
use getargs::Options;

use crate::utils::qemu::{self, QemuExitCode};

use super::command::Command;

pub struct ExitCommand {
    exit_code: QemuExitCode,
}

impl Command for ExitCommand {
    fn parse(args: alloc::vec::Vec<&str>) -> alloc::boxed::Box<dyn Command>
    where
        Self: Sized,
    {
        let mut opts = Options::new(args.iter().map(|&b| b));
        opts.next_positional();
        let mut exit_code = QemuExitCode::Success;
        if let Some(opt) = opts.next_positional() {
            match opt {
                "0" => exit_code = QemuExitCode::Success,
                "1" => exit_code = QemuExitCode::Failure,
                _ => {}
            }
        }
        Box::new(Self { exit_code })
    }

    fn execute(&self) {
        qemu::exit_qemu(self.exit_code)
    }

    fn name() -> alloc::string::String
    where
        Self: Sized,
    {
        String::from("qexit")
    }
}
