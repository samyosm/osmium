#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(slice_first_last_chunk)]

#[macro_use]
extern crate alloc;

mod memory;
mod panic;
mod terminal;
mod utils;

pub mod allocator;
pub mod commands;
pub mod events;
pub mod gdt;
pub mod interrupts;

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
    init();

    println!("OS: Start");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    hlt_loop();
}
