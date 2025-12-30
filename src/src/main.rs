#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(furst_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use furst_os;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    furst_os::println!("{}", info);
    furst_os::hlt_loop();
}
entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> !{
    #[cfg(test)]
    test_main();

    furst_os::println!("Hello World!");
    furst_os::init();
    furst_os::println!("It did not crash!");

    use x86_64::{structures::paging::Translate, VirtAddr};
    let offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { furst_os::memory::init(offset) };
    let addresses = [
        0x1,
        0xb2000,
        0xb43c999,
        0xb8000,
    ];
    for &addr in &addresses {
        let virt_addr = VirtAddr::new(addr);
        let phys = mapper.translate_addr(virt_addr);
        furst_os::println!("{:?} -> {:?}", virt_addr, phys);
    }
    furst_os::hlt_loop();
}
