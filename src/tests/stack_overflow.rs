#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
use core::panic::PanicInfo;
use lazy_static::lazy_static;
use x86_64::structures::idt::{
    InterruptDescriptorTable,
    InterruptStackFrame,
};
use furst_os;

#[panic_handler]
fn panic(info: &PanicInfo) ->! {
    furst_os::test_panic(info)
}

lazy_static!{
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault.set_handler_fn(test_df_handler)
                .set_stack_index(furst_os::gdt::DOUBLE_FAULT_INDEX);
        }
        idt
    };
}

extern "x86-interrupt" fn test_df_handler(
    _stack: InterruptStackFrame,
    _error_code: u64
) -> ! {
    furst_os::serial_println!("[ok]");
    furst_os::exit_qemu(furst_os::QemuExitCode::Success);
    loop{}
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    volatile::Volatile::new(0).read();
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    furst_os::serial_print!("stack_overflow::stack_overflow...\t");
    furst_os::gdt::init();
    TEST_IDT.load();
    stack_overflow();
    furst_os::serial_println!("[failed]");
    furst_os::exit_qemu(furst_os::QemuExitCode::Failed);
    loop {}
}
