use crate::println;
use x86_64::structures::idt::InterruptStackFrame;

#[allow(unused)]
pub extern "x86-interrupt" fn handler(_stack_frame: &mut InterruptStackFrame) {
    println!("EXCEPTION: TIMER");
}
