#![no_std]

#[macro_use]
mod kprint;
mod uart;

use core::ptr;

extern "C" {
    static bss_start: *const u64;
    static bss_end: *const u64;
}

#[no_mangle]
unsafe extern "C" fn kmain() {
    let end = ptr::read(bss_end);
    kprintln!("{:?}", end);
    kprintln!("hello");
    loop {}
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    kprintln!("{}", info);
    loop {}
}
