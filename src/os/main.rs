#![no_std]
#![no_main]
// Needed for testing
#![feature(custom_test_frameworks)]
#![test_runner(sylvan_shared::test::test_runner)]
#![reexport_test_harness_main = "test"]

use sylvan_shared::os::{init::init, vga::clear_screen};

mod panic;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();

    let mut balls = sylvan_shared::balls::create_balls();

    loop {
        clear_screen();

        for ball in balls.iter_mut() {
            ball.update();
        }

        for ball in balls.iter() {
            ball.draw();
        }

        for _ in 0..=5_000_000 {
            x86_64::instructions::nop();
        }
    }
}
