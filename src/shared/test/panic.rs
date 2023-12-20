use crate::{
    os::exit_qemu::{exit_qemu, QemuExitCode},
    serial_println,
};
use core::panic::PanicInfo;

pub fn panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);

    #[allow(clippy::empty_loop)]
    loop {}
}
