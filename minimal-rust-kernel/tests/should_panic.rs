#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use minimal_rust_kernel::{exit_qemu, serial_println, QemuExitCode};

#[no_mangle]
pub extern "C" fn _start() {
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

use minimal_rust_kernel::serial_print;

fn should_fail() {
    serial_print!("should_fail... ");
    assert_eq!(0, 1);
}
