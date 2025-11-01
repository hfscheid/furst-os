#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(furst_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use furst_os;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    furst_os::println!("{}", info);
    loop{}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> !{
    #[cfg(test)]
    test_main();

    furst_os::println!("Hello World!");
    furst_os::init();
    furst_os::println!("It did not crash!");
    loop {}
}
