use pic8259::ChainedPics;
use spin::Mutex;

const PIC_OFFSET: u8 = 8;
pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + PIC_OFFSET;

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
}

impl InterruptIndex {
    pub fn u8(self) -> u8 {
        self as u8
    }

    pub fn usize(self) -> usize {
        usize::from(self.u8())
    }
}
