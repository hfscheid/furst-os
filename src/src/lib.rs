#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

extern crate alloc;

// REEXPORTS
pub mod serial;
pub mod vga_buffer;
pub mod interrupt;
pub mod gdt;
pub mod memory;
pub mod allocator;
pub mod task;

// PANIC HANDLER
use core::panic::PanicInfo;
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic(info);
}

pub fn test_panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

// TYPES
pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T 
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t\t\t\t ", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed  = 0x11,
}

// FUNCTIONS
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(test)]
use bootloader::{BootInfo, entry_point};
#[cfg(test)]
entry_point!(test_kernel_main);
#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    test_main();
    hlt_loop();
}

pub fn init() {
    interrupt::init_idt();
    gdt::init();
    unsafe {
        interrupt::PICS.lock().initialize()
    };
    x86_64::instructions::interrupts::enable();
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests{
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn initial_assertion() {
    assert_eq!(1,1);
}
