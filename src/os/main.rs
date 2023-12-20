#![no_std]
#![no_main]
// Needed for testing
#![feature(custom_test_frameworks)]
#![test_runner(sylvan_shared::test::test_runner)]
#![reexport_test_harness_main = "test"]

use sylvan_shared::{os::init::init, print, println};

mod panic;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();

    // Test breakpoint exception
    x86_64::instructions::interrupts::int3();

    println!("I love you, Mom{}", "!");

    #[cfg(test)]
    test();

    print!("\nEntering useless loop");
    loop {
        print!(".");
        for _ in 0..=1_000_000 {
            x86_64::instructions::nop();
        }
    }
}
