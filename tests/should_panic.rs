#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test"]

use core::panic::PanicInfo;
use sylvan_shared::{
    os::{
        exit_qemu::{exit_qemu, QemuExitCode},
        halt_loop,
    },
    serial_println,
    test::{FailPrint, OkPrint},
};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    sylvan_shared::os::init::init();

    test();

    halt_loop();
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    for test in tests {
        test();

        serial_println!("should_panic::should_panic: {}", FailPrint);
        exit_qemu(QemuExitCode::Failed);
    }
    exit_qemu(QemuExitCode::Success);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("should_panic::should_panic: {}", OkPrint);
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
