#![feature(lang_items)]
#![no_std]

extern crate rlibc;
extern crate x86;
//extern crate time;

#[macro_use]
extern crate log;

//use x86::shared::control_regs::{cr0,cr0_write};
//use time::now;
use x86::shared::halt;

pub mod syslog;
pub mod arch;

#[no_mangle]
pub extern fn rust_main() {
    //let boot_time = now();
    arch::init();

    unsafe { halt(); }
}

// This is required by because Rust creates some references to it even if we disable it.
#[no_mangle] fn _Unwind_Resume() -> ! { loop {} }
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop{} }
