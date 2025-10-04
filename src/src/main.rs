#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

mod vga_buffer;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> !{
    #[cfg(test)]
    {
        test_main();
        loop {}
    }
    println!("Hello World! The answer is {}", 42);
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests{
        test();
    }
}
