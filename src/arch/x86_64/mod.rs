// X86_64-specific code.

pub struct CPU {
	pub arch: &'static str,
}

impl CPU {
	pub fn init() -> CPU {
		//println!("Architecture is x86_64.");
		CPU { arch: "x86_64" }
	}
}