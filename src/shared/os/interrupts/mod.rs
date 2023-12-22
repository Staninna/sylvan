pub mod exeption_handlers;
pub mod hardware;
pub mod idt;

mod tests {
    #[test_case]
    fn test_breakpoint_exception() {
        x86_64::instructions::interrupts::int3();
    }
}
