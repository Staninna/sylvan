use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::println;
    use sylvan_shared::os::halt_loop;

    println!("{}", info);
    halt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    sylvan_shared::test::panic_handler(info)
}
