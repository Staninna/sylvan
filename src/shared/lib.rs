#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test"]

// Modules for non-test code
pub mod serial;
pub mod vga;

// Module for testing
pub mod exit_qemu;
pub mod test;

// All code below is for testing this library

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test();

    #[allow(clippy::empty_loop)]
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    test::panic_handler(info)
}