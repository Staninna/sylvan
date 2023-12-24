#![no_std]
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test"]

pub mod os;
pub mod test;
pub mod verlet;

// All code below is for testing this library

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use os::halt_loop;

    os::init::init();

    test();

    halt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    test::panic_handler(info)
}
