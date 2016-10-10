// X86_64-specific code.
extern crate x86;
extern crate multiboot2;

//use self::multiboot2;
use self::x86::shared::halt;

pub struct CPU {
	pub arch: &'static str,
	pub memory_areas: &'static multiboot2::MemoryMapTag,
}

impl CPU {
	pub fn init(boot_info_struct: usize) -> Self {
		let boot_info = unsafe { multiboot2::load(boot_info_struct) };
		let memory_map_tag = boot_info.memory_map_tag()
			.expect("Memory map tag required");

		for area in memory_map_tag.memory_areas() {
			println!("memory region 0x{:x} length 0x{:x}", area.base_addr, area.length);
		}
			
		CPU { arch: "x86_64", memory_areas: memory_map_tag }
	}
	
	#[inline(always)]
	pub fn halt() {
		unsafe { halt(); }
	}
	
	
}