#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(minimal_rust_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use minimal_rust_kernel::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    minimal_rust_kernel::init();
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // };

    // fn stack_overflow() {
    //     stack_overflow();
    // }

    // stack_overflow();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    minimal_rust_kernel::test_panic_handler(info)
}