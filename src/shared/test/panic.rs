use crate::exit_qemu::{exit_qemu, QemuExitCode};
use core::panic::PanicInfo;

pub fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}
