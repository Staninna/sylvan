use crate::{
    os::{
        exit_qemu::{exit_qemu, QemuExitCode},
        halt_loop,
    },
    serial_println,
    test::FailPrint,
};
use core::panic::PanicInfo;

pub fn panic_handler(info: &PanicInfo) -> ! {
    serial_println!("{}", FailPrint);
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);

    halt_loop();
}
