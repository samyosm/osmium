#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

mod color;
pub mod gdt;
pub mod interrupts;
mod panic;
mod utils;
mod write;

use interrupts::setup;

pub fn init() {
    gdt::init();
    setup::init_idt();
    unsafe { setup::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    init();

    println!("Hello, world!");

    hlt_loop();
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    // exit_qemu(QemuExitCode::Success); // TODO: Implement printing to the host console
}
