use super::TestableFn;
use crate::{
    os::exit_qemu::{exit_qemu, QemuExitCode},
    serial_print, serial_println,
    test::NoTestsPrint,
};

pub fn test_runner(tests: &[&dyn TestableFn]) {
    let count = tests.len();

    if count == 0 {
        serial_println!("{}", NoTestsPrint);
        exit_qemu(QemuExitCode::Success);
    }

    serial_println!("Running {} tests", count);

    for (idx, test) in tests.iter().enumerate() {
        serial_print!("{:>3}/{:<3}", idx + 1, count);
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}
