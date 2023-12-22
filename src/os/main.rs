#![no_std]
#![no_main]
// Needed for testing
#![feature(custom_test_frameworks)]
#![test_runner(sylvan_shared::test::test_runner)]
#![reexport_test_harness_main = "test"]

use arrayvec::ArrayString;
use sylvan_shared::{os::init::init, print, println};

mod panic;

const BUFFER_CAP: usize = 25 * 80;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();

    let rick = include_str!("../../ascii_art.txt");

    loop {
        // Print each line of the ascii art
        let mut buffer: ArrayString<BUFFER_CAP> = ArrayString::new();
        for line in rick.lines() {
            // If the line has - in it, wait 1000/30 ms
            if line.contains('-') {
                buffer.push_str(line);
                print!("{}", buffer);
                buffer.clear();
                for _ in 0..100_000_000 {
                    x86_64::instructions::nop();
                }
                continue;
            } else {
                buffer.push_str(line);
            }
        }
    }
}
