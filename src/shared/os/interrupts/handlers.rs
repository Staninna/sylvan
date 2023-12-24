use crate::{
    format,
    os::interrupts::hardware::{InterruptIndex, PICS},
    os::rng::SEED,
    print, println,
};
use lazy_static::lazy_static;
use pc_keyboard::{layouts::Us104Key, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::{instructions::port::Port, structures::idt::InterruptStackFrame};

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

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<Us104Key, ScancodeSet1>> = Mutex::new(Keyboard::new(
        ScancodeSet1::new(),
        Us104Key,
        HandleControl::Ignore
    ));
}

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(DecodedKey::Unicode(key)) = keyboard.process_keyevent(key_event) {
            // Modify the RNG seed if a number is pressed
            // cuz we so low level only way to get random input is from the keyboard
            let is_number = key.is_numeric();
            if is_number {
                let seed_as_str = format!("{}", unsafe { SEED });
                unsafe {
                    SEED = match format!("{}{}", seed_as_str, key).parse() {
                        Ok(seed) => {
                            print!("{}", key);
                            seed
                        }
                        Err(_) => SEED,
                    };
                }
            }
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.u8());
    }
}

// Format macro
#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => ($crate::os::interrupts::handlers::_format(format_args!($($arg)*)));
}

pub fn _format(args: core::fmt::Arguments) -> arrayvec::ArrayString<512> {
    use core::fmt::Write;

    let mut s = arrayvec::ArrayString::<512>::new();
    s.write_fmt(args).unwrap();
    s
}
