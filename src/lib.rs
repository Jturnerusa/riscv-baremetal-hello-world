#![no_std]

#[macro_use]
mod kprint;
mod uart;

#[no_mangle]
extern "C" fn kmain() {
    loop {}
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    kprintln!("{}", info);
    loop {}
}
