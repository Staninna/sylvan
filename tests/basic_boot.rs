#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(sylvan_shared::test::test_runner)]
#![reexport_test_harness_main = "test"]

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test();

    #[allow(clippy::empty_loop)]
    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    sylvan_shared::test::panic_handler(info)
}
