use crate::{
    arch::{device::irqchip::hlic, time},
    dtb::DTB_BINARY,
};
use fdt::{
    node::{FdtNode, NodeProperty},
    Fdt,
};

pub mod cpu;
pub(crate) mod irqchip;
pub mod serial;

use crate::arch::device::irqchip::init_clint;

unsafe fn init_intc(cpu: &FdtNode) {
    let intc_node = cpu
        .children()
        .find(|x| x.name == "interrupt-controller")
        .unwrap();
    assert_eq!(intc_node.compatible().unwrap().first(), "riscv,cpu-intc");
    // This controller is hardwired into interrupt handler code and has no Mmios
    hlic::init(); // enable interrupts at HLIC level
}

pub unsafe fn init() {
    unsafe {
        let data = DTB_BINARY.get().unwrap();
        let fdt = Fdt::new(data).unwrap();

        crate::dtb::irqchip::init(&fdt);

        let cpu = fdt.find_node(format!("/cpus/cpu@{}", 0).as_str()).unwrap();
        init_intc(&cpu);
        init_time(&fdt);
    }
}

fn init_time(fdt: &Fdt) {
    let cpus = fdt.find_node("/cpus").unwrap();
    let clock_freq = cpus
        .property("timebase-frequency")
        .unwrap()
        .as_usize()
        .unwrap();
    time::init(clock_freq);
}

pub unsafe fn init_noncore() {
    unsafe {
        let data = DTB_BINARY.get().unwrap();
        let fdt = Fdt::new(data).unwrap();

        init_clint(&fdt);
        serial::init(&fdt);
    }
}

pub struct ArchPercpuMisc;

impl ArchPercpuMisc {
    pub const fn default() -> Self {
        Self
    }
}
