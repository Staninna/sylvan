#![cfg(test)]

use crate::{
    exit_qemu::{exit_qemu, QemuExitCode},
    {serial_print, serial_println},
};

struct Green(&'static str);
struct Red(&'static str);

impl core::fmt::Display for Green {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "\x1b[32m{}\x1b[0m", self.0)
    }
}

impl core::fmt::Display for Red {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "\x1b[31m{}\x1b[0m", self.0)
    }
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    serial_print!("trivial assertion... ");
    assert_eq!(1, 1);
    serial_println!("{}", Green("[ok]"));
}
