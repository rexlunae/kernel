#[macro_use]
pub mod vga;

// Porting:
// Each arch will export these classes:
//
//  System: A class for controlling the whole system.
//      ::init() - Run once at boot.  Sets up anything that is needed to get going.
//
//  Events: A class for handling various hardware-defined events, such as IRQs and traps.
//      ::init() - Runs once at boot.  Sets up event handlers.
//
//  Heap: A memory allocator.  XXX Does this go here?
//
//  VMM: A class for controlling virtual memory.
//      ::page_sizes - A list of valid page sizes.
//      ::map(virtual_address, physical_address, page_size) - Maps a physical page to a virtual
//      address.

#[cfg(target_arch="x86_64")]
pub mod x86_64;
#[cfg(target_arch="x86_64")]
pub use self::x86_64::{System,Events,Heap,VMM};

// Initialize the architecture-specific features.
pub fn init(boot_info_struct: usize) -> System {
	let sys = System::init(boot_info_struct);
    Events::init();
	
	sys
}

#[inline(always)]
pub fn halt() {
	System::halt();
}
