use lazy_static::lazy_static;
use x86_64::{
    instructions::{
        segmentation::{Segment, CS},
        tables::load_tss,
    },
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
        tss::TaskStateSegment,
    },
    VirtAddr,
};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

// The TSS is a data structure used by the CPU to find the stack for interrupts and exceptions for stack switching.
lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();

        // The stack for double faults is a separate stack that is only used when a double fault occurs.
        // This is necessary because the normal interrupt stack might be full, so the CPU cannot push any values to it.
        // The double fault stack is never used otherwise, so it is empty most of the time.
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = (1024 * 4) * 5; // 5 pages of 4 KiB
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            stack_start + STACK_SIZE // stack_end
        };
        tss
    };
}

// The GDT is a data structure used by the CPU to find the segment for memory accesses.
lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();

        // The kernel code segment is a segment that is used for kernel code. It is read-only and executable.
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());

        // The TSS segment is a segment that is used for stack switching. It is read-only and not executable.
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (
            gdt,
            Selectors {
                code_selector,
                tss_selector,
            },
        )
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init() {
    GDT.0.load();

    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}
