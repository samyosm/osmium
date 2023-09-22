use core::panic::PanicInfo;

use crate::{eprintln, hlt_loop};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    eprintln!("{}", info);

    hlt_loop();
}
