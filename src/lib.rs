#![no_std]

mod uart {
    const ADDRESS: usize = 0x10000000;

    pub fn write(message: &str) {
        let ptr = ADDRESS as *mut u8;
        for byte in message.bytes() {
            unsafe {
                core::ptr::write_volatile(ptr, byte);
            }
        }
    }
}

#[no_mangle]
extern "C" fn kmain() {
    uart::write("hello world from rust");
    loop {}
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    loop {}
}
