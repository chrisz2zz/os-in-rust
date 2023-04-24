#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(minimal_rust_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use minimal_rust_kernel::println;

use bootloader::{entry_point, BootInfo};
use x86_64::{structures::paging::PageTable, VirtAddr};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use minimal_rust_kernel::memory::translate_addr;
    println!("Hello World{}", "!");

    minimal_rust_kernel::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let addresses = [
        0xb8000,
        0x201008,
        0x0100_0020_1a10,
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = unsafe {
            translate_addr(virt, phys_mem_offset)
        };
        println!("{:?} -> {:?}", virt, phys);
    }

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    minimal_rust_kernel::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    minimal_rust_kernel::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    minimal_rust_kernel::test_panic_handler(info)
}
