pub fn init() {
    super::gdt::init();
    super::interrupts::init_idt();
}
