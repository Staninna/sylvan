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

    println!("Hello, to my calculator!");
    println!("Type in some numbers and operators and press enter to evaluate.");
    println!("Press backspace to remove the last character.");
    println!("Allowed characters: 0-9, +, -, *, /, Enter, Backspace");
    println!("Press the power button to exit.");
    println!("One last thing i can't handle floating point numbers");
    for _ in 0..2 {
        println!();
    }

    halt_loop();
}
