use crate::os::{
    gdt,
    interrupts::{
        handlers::{
            breakpoint_exception_handler, double_fault_exception_handler, timer_interrupt_handler,
        },
        hardware::InterruptIndex,
    },
};
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

// The IDT is a data structure used by the CPU to find the handler for interrupts and exceptions.
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // Exceptions
        idt.breakpoint.set_handler_fn(breakpoint_exception_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_exception_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        // Hardware Interrupts
        idt[InterruptIndex::Timer.usize()].set_handler_fn(timer_interrupt_handler);

        idt
    };
}

pub fn init() {
    IDT.load();
}
