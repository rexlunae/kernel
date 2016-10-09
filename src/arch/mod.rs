#[macro_use]
pub mod vga;

#[cfg(target_arch="x86_64")]
mod x86_64;
type CPU = x86_64::CPU;

// Initialize the architecture-specific features.
pub fn init() -> CPU {
	let cpu = CPU::init();
	
	cpu
}

#[inline(always)]
pub fn halt() {
	unsafe { CPU::halt(); }
}