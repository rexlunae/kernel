pub mod vga;

// Initialize the architecture-specific features.
pub fn init() {
    let hello = b"Hello World!";
    let color_byte = 0x1f;

    let mut hello_colored = [color_byte; 24];
    for (i, char_byte) in hello.into_iter().enumerate() {
        hello_colored[i * 2] = *char_byte;
    }

    let buffer_ptr = (0xb8000  +1988) as *mut _;
    unsafe { *buffer_ptr = hello_colored };
    
}
