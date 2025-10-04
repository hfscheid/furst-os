#![no_std]
#![no_main]

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

mod vga_buffer;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> !{
    println!("Hello World! The answer is {}", 42);
    loop {}
}
