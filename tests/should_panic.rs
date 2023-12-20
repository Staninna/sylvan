#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test"]

use core::panic::PanicInfo;
use sylvan_shared::{
    os::exit_qemu::{exit_qemu, QemuExitCode},
    serial_println,
    test::{FailPrint, OkPrint},
};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    sylvan_shared::os::init::init();

    test();

    #[allow(clippy::empty_loop)]
    loop {}
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running should_panic test");
    for test in tests {
        test();

        serial_println!("test should_panic passed: {}", FailPrint);
        exit_qemu(QemuExitCode::Failed);
    }
    exit_qemu(QemuExitCode::Success);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("test should_panic failed: {}", OkPrint);
    exit_qemu(QemuExitCode::Success);

    loop {}
}

#[allow(unused)]
#[test_case]
fn should_panic() {
    let no = false;
    let yes = true;
    assert!(no);
}
