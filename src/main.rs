#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

extern crate alloc;

pub mod allocator;
mod color;
pub mod gdt;
pub mod interrupts;
mod memory;
mod panic;
mod utils;
mod write;

use alloc::vec::Vec;
use bootloader::{entry_point, BootInfo};
use interrupts::setup;
use x86_64::VirtAddr;

use crate::memory::BootInfoFrameAllocator;

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

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    #[cfg(test)]
    test_main();

    init();

    println!("OS: Start");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let mut vec = Vec::new();

    vec.push("Hello, world!");
    vec.push("Hello, world!");
    vec.push("Hello, world!");
    vec.push("Hello, world!");

    for str in vec {
        println!("{str}");
    }

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
