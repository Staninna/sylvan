#![no_std]
#![no_main]
// Needed for testing
#![feature(custom_test_frameworks)]
#![test_runner(sylvan_shared::test::test_runner)]
#![reexport_test_harness_main = "test"]

use sylvan_shared::{
    os::{halt_loop, init::init},
    println,
};

mod panic;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();

    println!("Hello, Mom and Dad!");
    println!("I love you both very much!");

    #[cfg(test)]
    test();

    halt_loop();
}
