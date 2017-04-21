#![feature(allocator)]
#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![feature(abi_x86_interrupt)]
#![no_std]
extern crate rlibc;
extern crate spin;
//extern crate time;

#[macro_use]
extern crate lazy_static;


#[macro_use]
extern crate log;

#[macro_use]
//extern crate alloc_no_stdlib;
extern crate linked_list_allocator;


//use time::now;

pub mod syslog;
pub mod term;
pub mod memory;
pub mod process;
//pub mod interrupts;

// arch is what gives us a console to write to.
#[macro_use]
pub mod arch;

// This function runs unprivileged.
pub extern "C" fn userland_main() {


}

impl arch::Events {
    pub fn test() {
    }
}

#[no_mangle]
pub extern fn rust_main(boot_info_struct: usize) {
    let sys = arch::init(boot_info_struct);
    //let boot_time = now();
    println!("CPU Architecture: {}", sys.arch);
    println!("Boot struct located at 0x{:x}", boot_info_struct);
    
    // Main Kernel loop
    loop {
		arch::halt();
		println!("Cycle!");
    }
}

// This is required by because Rust creates some references to it even if we disable it.
#[allow(non_snake_case)]
#[no_mangle] pub fn _Unwind_Resume() -> ! { loop {} }
#[lang = "eh_personality"] extern fn eh_personality() {}
#[no_mangle] #[lang = "panic_fmt"] extern fn panic_fmt() -> ! { println!("KERNEL PANIC!!!"); loop{} }
