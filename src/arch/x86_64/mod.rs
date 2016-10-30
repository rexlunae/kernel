// X86_64-specific code.
extern crate x86;
extern crate multiboot2;

//use self::multiboot2;
use self::x86::bits64::paging;
use self::x86::shared::halt;
use self::x86::shared::control_regs::{cr0,cr3,cr4};
//use self::alloc_no_stdlib::Allocator;

extern "C" {
	static mut p4_table: u64; //&'static mut [paging::PAddr; 512];
	static mut p3_table: u64; //&'static mut [paging::PAddr; 512];
	static mut p2_table: u64; //&'static mut [paging::PAddr; 512];

	//static mut p4_table: paging::PML4;
	//static mut p3_table: paging::PDPT;
	//static mut p2_table: paging::PD;

}
pub struct CPU {
	pub arch: &'static str,
	pub memory_areas: &'static multiboot2::MemoryMapTag,
	pub elf_sections: &'static multiboot2::ElfSectionsTag,
}

impl CPU {
	pub fn init(boot_info_struct: usize) -> Self {
		let boot_info = unsafe { multiboot2::load(boot_info_struct) };

		let elf_sections = boot_info.elf_sections_tag()
			.expect("Elf-sections tag required");
		for section in elf_sections.sections() {
			println!("kernel section 0x{:x}, size 0x{:x}, flags 0x{:x}", section.addr, section.size, section.flags)
		}

		let memory_map_tag = boot_info.memory_map_tag()
			.expect("Memory map tag required");
		for area in memory_map_tag.memory_areas() {
			println!("memory region 0x{:x} length 0x{:x}", area.base_addr, area.length);
		}

		unsafe {
			println!("cr0: 0b{:b}, cr4: 0b{:b}", cr0().bits(), cr4().bits());
			println!("cr3: 0x{:x}", cr3());
			println!("p4_table @ {:p}: 0x{:x}", &p4_table, p4_table);
			println!("p3_table @ {:p}: 0x{:x}", &p3_table, p3_table);
			println!("p2_table @ {:p}: 0x{:x}", &p2_table, p2_table);

		}

		CPU { arch: "x86_64", memory_areas: memory_map_tag, elf_sections: elf_sections }
		
	}
	
	#[inline(always)]
	pub fn halt() {
		unsafe { halt(); }
	}
	
	
}
