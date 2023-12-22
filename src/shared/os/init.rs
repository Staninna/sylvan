pub fn init() {
    // Initialize the Global Descriptor Table (GDT) and the Task State Segment (TSS)
    super::gdt::init();

    // Initialize the Interrupt Descriptor Table (IDT)
    super::interrupts::init_idt();

    // Initialize the PICs (Programmable Interrupt Controllers)
    unsafe { super::interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}
