use core::panic::PanicInfo;

use crate::{eprintln, hlt_loop, utils::write_macros::INPUT};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let b = info.location().unwrap().line() as u8 + 50;
    INPUT.lock().input_char(b);
    eprintln!("{}", info);

    hlt_loop();
}
