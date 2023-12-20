#![no_std]
#![no_main]
// Needed for testing
#![feature(custom_test_frameworks)]
#![test_runner(sylvan_shared::test::test_runner)]
#![reexport_test_harness_main = "test"]

use sylvan_shared::println;

mod panic;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("I love you, Mom{}", "!");

    #[cfg(test)]
    test();

    #[allow(clippy::empty_loop)]
    loop {}
}
