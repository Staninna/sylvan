use crate::{serial_print, serial_println, test::OkPrint};

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{:88}   ", core::any::type_name::<T>());
        self();
        serial_println!("{}", OkPrint);
    }
}
