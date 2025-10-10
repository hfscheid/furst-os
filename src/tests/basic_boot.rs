#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(furst_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use furst_os;
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();
    loop{}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    furst_os::test_panic(info);
}
