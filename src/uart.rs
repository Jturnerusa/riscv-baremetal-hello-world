use core::fmt;
use core::ptr;

pub struct Uart {
    address: usize,
}

impl Uart {
    pub unsafe fn new(address: usize) -> Self {
        Self { address }
    }
}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let p = self.address as *mut u8;
        for byte in s.bytes() {
            unsafe { ptr::write_volatile(p, byte) };
        }
        Ok(())
    }
}
