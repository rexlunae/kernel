ENTRY(start)

SECTIONS {
  /* sets load address to MiB (convention) */
  . = 1M;

  .boot : {
    /* put the header at the beginning */
    KEEP(*(.multiboot_header))
  }

    .text : {
        *(.text .text.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
    }

    .data.rel.ro : {
        *(.data.rel.ro.local*) *(.data.rel.ro .data.rel.ro.*)
    }
}
