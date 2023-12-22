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

    println!("I love you, Mom{}", "!");
    println!("I also love you, Dad{}", "!");

    #[cfg(test)]
    test();

    loop {
        for _ in 0..1000000 {
            x86_64::instructions::nop();
        }
        print!("-");
    }
}
