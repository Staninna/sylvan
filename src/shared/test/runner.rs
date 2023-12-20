use super::Testable;
use crate::exit_qemu::{exit_qemu, QemuExitCode};

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
