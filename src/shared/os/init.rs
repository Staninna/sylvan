pub fn init() {
    super::gdt::init();
    super::interrupts::init_idt();

    unsafe { super::interrupts::PICS.lock().initialize() };
}
