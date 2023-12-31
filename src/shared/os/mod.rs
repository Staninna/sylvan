pub mod exit_qemu;
pub mod gdt;
pub mod init;
pub mod interrupts;
pub mod serial;
pub mod vga;

pub fn halt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
