extern crate x86_64;

use core::mem::size_of;

use self::x86_64::structures::idt::{Idt,IdtEntry,HandlerFunc,HandlerFuncWithErrCode,ExceptionStackFrame};

extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut ExceptionStackFrame, _error_code: u64)
{
        println!("\nEXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
            loop {}
}

extern "x86-interrupt" fn default_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("handled interrupt!!!")
}

lazy_static! {
    static ref IDT: Idt = {
        //println!("Made it into the loader.");
        let mut idt = Idt::new();

        idt.breakpoint.set_handler_fn(default_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);

        idt
    };
}

pub fn init() {
    println!("Loading IDT...");
    IDT.load();
    println!("Loaded IDT.");
}
