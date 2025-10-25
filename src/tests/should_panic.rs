#![no_std]
#![no_main]

use furst_os;
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    should_panic();
    furst_os::serial_println!("[test did not panic]");
    furst_os::exit_qemu(furst_os::QemuExitCode::Failed);
    loop {}
}

fn should_panic() {
    furst_os::serial_print!("should_panic::should_panic...\t");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    furst_os::serial_println!("[ok]");
    furst_os::exit_qemu(furst_os::QemuExitCode::Success);
    loop {}
}
