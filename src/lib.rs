#![no_std]

#[macro_use]
mod kprint;
mod uart;

extern "C" {
    static heap_start: *const ();
    static heap_end: *const ();
}

#[no_mangle]
unsafe extern "C" fn kmain() {
    loop {}
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    kprintln!("{}", info);
    loop {}
}
