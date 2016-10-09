// X86_64-specific code.
extern crate x86;

use self::x86::shared::halt;

pub struct CPU {
	pub arch: &'static str,
}

impl CPU {
	pub fn init() -> Self {
		CPU { arch: "x86_64" }
	}
	
	#[inline(always)]
	pub fn halt() {
		unsafe { halt(); }
	}
}