pub fn init() {
    // Initialize the Global Descriptor Table (GDT) and the Task State Segment (TSS)
    super::gdt::init();

    // Initialize the Interrupt Descriptor Table (IDT)
    super::interrupts::exeptions::init_idt();

    // Initialize the PICs (Programmable Interrupt Controllers)
    unsafe { super::interrupts::hardware::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}
