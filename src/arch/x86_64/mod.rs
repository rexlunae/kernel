// X86_64-specific code.
extern crate x86;
extern crate x86_64;
extern crate multiboot2;

use core::mem::size_of;
use core::slice::from_raw_parts_mut;

//use self::multiboot2;
use self::x86::bits64::paging;
use self::x86::shared::halt;
use self::x86::shared::control_regs::{cr0,cr3_write,cr3,cr4};
use self::x86::shared::tlb;

// Whatever we use for our heap, we want to reexport it.
pub use linked_list_allocator::Heap;

use self::x86_64::instructions::segmentation;
use self::x86_64::structures::gdt::SegmentSelector;
use self::x86_64::PrivilegeLevel;

mod idt;



pub enum Events {
    Syscall,        // Software call to kernel mode.
    IRQ(u8),        // Hardware call from outside.
    Trap(u8),       // Software fault.  Typically from userland.
    Runlevel(i8),   // Internally-generated.
}



impl Events {
    pub fn syscall() -> Self { Events::Syscall }
    pub fn irq(value: u8) -> Self { Events::IRQ(value) }
    pub fn trap(value: u8) -> Self { Events::Trap(value) }
    pub fn runlevel(value: i8) -> Self { Events::Runlevel(value) }

    pub fn generate(event: Self) {
    }

    pub fn get(&self) -> Self {
        // 1. Add to scheduler's queue
        // 2. run scheduler
        Events::Syscall
    }

    pub fn init() {


    }
}


pub struct VMM {
}



//use self::alloc_no_stdlib::Allocator;

extern "C" {
	static mut p4_table: paging::PML4;
	static mut p3_table: paging::PDPT;
	static mut p2_table: paging::PD;
	// The kernel doesn't use a page table because it's using large (2M) pages.

}

//pub struct VMMBase (PAddr);

//#[derive()]
pub struct System {
	pub arch: &'static str,
	//pub heap: &'static mut Heap,
	pub memory_areas: &'static multiboot2::MemoryMapTag,
	pub elf_sections: &'static multiboot2::ElfSectionsTag,
}

impl System {
	pub fn init(boot_info_struct: usize) -> Self {
		let boot_info = unsafe { multiboot2::load(boot_info_struct) };

        idt::init();

		let elf_sections = boot_info.elf_sections_tag()
			.expect("Elf-sections tag required");

		let memory_map_tag = boot_info.memory_map_tag()
			.expect("Memory map tag required");
		
		//let mut heap = Heap::empty();
		'a: for area in memory_map_tag.memory_areas() {
			//println!("memory region 0x{:x} length 0x{:x}", area.base_addr, area.length);
			
			// We don't want to clobber the kernel, so we have to look for the sections in the memory regions and explicitly exclude them from the heap.
			// In some cases, this could mean splitting a region into multiple heaps.
			let mut current_base = area.base_addr;
			
			'b: while current_base < (area.base_addr + area.length) {
				// Each time we restart, we need to reset the limit.
				let mut current_length = area.length - (current_base - area.base_addr);
				
				'c: for section in elf_sections.sections() {

					// Handle a section that begins at or before the memory region...
					if section.addr <= current_base {

						// ...and covers the entire memory region.  Just move on to the next memory region.
						if (section.addr + section.size) > (current_base + current_length) { continue 'a }
						// ...and ends inside the memory region.  Move the current base up.
						else if (section.addr + section.size) > current_base { current_base = section.addr + section.size }
					}
					
					// Handle a section that begins after the beginning of the memory region...
					if section.addr >= current_base {
						// ...but before the end
						if section.addr < current_base + current_length { current_length = section.addr - current_base }
					}
					// println!("kernel section 0x{:x}, size 0x{:x}, flags 0x{:x}", section.addr, section.size, section.flags)
				}
				
				// Create a heap for the new area, and add it to the existing heap.
				if current_length > size_of::<Heap>() as u64 {
//					println!("Adding new heap from 0x{:x}, size 0x{:x}", current_base, current_length);
					//let &mut current_ref = from_raw_parts_mut(current_base as *mut u8, current_length as usize);
					
					//let mut new_heap = unsafe { Heap::init(current_base as *mut u8, current_length as usize) };
					//let mut new_heap = unsafe { Heap::init(current_ref) };
					//println!('1');
					//println!("Is the heap empty: {}", heap.is_empty());
//					println!("Heap size: 0x{:x} ", size_of::<Heap> as usize);
					//if !heap.is_empty() {
                    if false {
						//println!('0');
						//println!("Is allocate_first_fit None? {}", heap.allocate_first_fit(0, 8).is_none());
						//println!('5');
						//println!("Allocated: {:p}", heap.allocate_first_fit(35, 8).expect("Allocating Heap"));
					}
					//println!(':');
					//unsafe { heap.add_memory(current_base as *mut u8, current_length as usize); }
					//println!('2');
				}
				
				current_base += current_length;
//				println!("current_base: 0x{:x}, area.base: 0x{:x}, area.length: 0x{:x}, top_addr: 0x{:x}", current_base, area.base_addr, area.length, area.base_addr + area.length);
			}
		}
		
		// Remap the kernel to use identity-mapped access to the first 512GB of memory.  For most configurations, this should be plenty.
		//println!("p3_table length: {}", unsafe { p3_table.len() });
		for index in 0..unsafe { p3_table.len() } {
			unsafe {
				p3_table[index] = paging::PDPTEntry::new(paging::PAddr::from_u64(index as u64 * 0x40000000),  paging::PDPT_P | paging::PDPT_RW | paging::PDPT_PS);
				tlb::flush_all();
			}
		}
		
		unsafe {
			println!("cr0: 0b{:b}, cr4: 0b{:b}", cr0().bits(), cr4().bits());
			println!("cr3: 0x{:x}", cr3());
			println!("p4_table @ {:p}: 0x{:x}", &p4_table, p4_table[0].bits());
			println!("p3_table[0] @ {:p}: 0x{:x}", &p3_table, p3_table[0].bits());
			//println!("p3_table[1] @ {:p}: 0x{:x}", &p3_table, p3_table[1].bits());
			//println!("p2_table[0] @ {:p}: 0x{:x}", &p2_table, p2_table[0].bits());
			//println!("p2_table[1] @ {:p}: 0x{:x}", &p2_table, p2_table[1].bits());

		}

		System { arch: "x86_64", memory_areas: memory_map_tag, elf_sections: elf_sections, /*heap: heap*/ }
		
	}
	
	#[inline(always)]
	pub fn halt() {
		unsafe { halt(); }
	}
	
	
}
