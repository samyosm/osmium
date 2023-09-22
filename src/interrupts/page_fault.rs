use crate::{eprintln, hlt_loop};
use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

pub extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    eprintln!("EXCEPTION: PAGE FAULT");
    eprintln!("Accessed Address: {:?}", Cr2::read());
    eprintln!("Error Code: {:?}", error_code);
    eprintln!("{:#?}", stack_frame);
    hlt_loop();
}
