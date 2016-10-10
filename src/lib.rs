#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![no_std]
extern crate rlibc;
extern crate spin;
//extern crate time;

#[macro_use]
extern crate log;

//use time::now;

pub mod syslog;
pub mod term;

// arch is what gives us a console to write to.
#[macro_use]
pub mod arch;


#[no_mangle]
pub extern fn rust_main(boot_info_struct: usize) {
    //let boot_time = now();
    let cpu = arch::init(boot_info_struct);
    println!("CPU Architecture: {}", cpu.arch);
    println!("Boot struct located at {}", boot_info_struct);
    
    // Main Kernel loop
    loop {
		unsafe { arch::halt(); }
		println!("Cycle!");
    }
}

// This is required by because Rust creates some references to it even if we disable it.
#[allow(non_snake_case)]
#[no_mangle] pub fn _Unwind_Resume() -> ! { loop {} }
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop{} }
