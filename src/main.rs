#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod color;
mod write;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    eprintln!("{}", info);

    loop {}
}
