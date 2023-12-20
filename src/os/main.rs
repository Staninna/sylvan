#![no_std]
#![no_main]
// Needed for testing
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test"]

mod exit_qemu;
mod panic;
mod serial;
mod test;
mod vga;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("I love you, Mom{}", "!");

    #[cfg(test)]
    test();

    #[allow(clippy::empty_loop)]
    loop {}
}
