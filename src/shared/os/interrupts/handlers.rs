use crate::{
    os::interrupts::hardware::{InterruptIndex, PICS},
    print, println,
};
use x86_64::structures::idt::InterruptStackFrame;

pub extern "x86-interrupt" fn breakpoint_exception_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn double_fault_exception_handler(
    stack_frame: InterruptStackFrame,
    _: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // print!(".");

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.u8());
    }
}

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };

    let key = match scancode {
        0x1e => Some('a'),  // A
        0x30 => Some('b'),  // B
        0x2e => Some('c'),  // C
        0x20 => Some('d'),  // D
        0x12 => Some('e'),  // E
        0x21 => Some('f'),  // F
        0x22 => Some('g'),  // G
        0x23 => Some('h'),  // H
        0x17 => Some('i'),  // I
        0x24 => Some('j'),  // J
        0x25 => Some('k'),  // K
        0x26 => Some('l'),  // L
        0x32 => Some('m'),  // M
        0x31 => Some('n'),  // N
        0x18 => Some('o'),  // O
        0x19 => Some('p'),  // P
        0x10 => Some('q'),  // Q
        0x13 => Some('r'),  // R
        0x1f => Some('s'),  // S
        0x14 => Some('t'),  // T
        0x16 => Some('u'),  // U
        0x2f => Some('v'),  // V
        0x11 => Some('w'),  // W
        0x2d => Some('x'),  // X
        0x15 => Some('y'),  // Y
        0x2c => Some('z'),  // Z
        0x39 => Some(' '),  // Space
        0x1c => Some('\n'), // Enter
        _ => None,
    };
    if let Some(key) = key {
        print!("{}", key);
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.u8());
    }
}
