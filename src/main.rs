#![no_std]
#![no_main]

mod panic;
mod vga;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("I love you, Mom{}", "!");

    loop {}
}
