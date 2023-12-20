#![cfg(test)]

use crate::{
    exit_qemu::{exit_qemu, QemuExitCode},
    {serial_print, serial_println},
};

pub struct Ok;
pub struct Fail;

impl core::fmt::Display for Ok {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "\x1b[32m{}\x1b[0m", "[ok]")
    }
}

impl core::fmt::Display for Fail {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "\x1b[31m{}\x1b[0m", "[fail]")
    }
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{:60}", core::any::type_name::<T>());
        self();
        serial_println!("{}", Ok);
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    let count = tests.len();
    if count == 0 {
        serial_println!("No tests to run!");
        exit_qemu(QemuExitCode::Success);
    }

    serial_println!("Running {} tests", count);

    for (idx, test) in tests.iter().enumerate() {
        serial_print!("{:>3}/{:<3}", idx + 1, count);
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn dummy_test() {
    assert!(true);
}
