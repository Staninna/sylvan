pub(super) mod handlers;
pub mod hardware;
pub mod idt;
pub(super) mod keyboard_handler;

mod tests {
    #[test_case]
    fn test_breakpoint_exception() {
        x86_64::instructions::interrupts::int3();
    }
}
