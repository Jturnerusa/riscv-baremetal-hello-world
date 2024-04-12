#[cfg(qemu)]
macro_rules! kprintln {
    ($fmt:literal, $($e:expr),+) => {
        {
            use ::core::fmt::Write;
            const QEMU_UART_ADDRESS: usize = 0x10000000;
            let mut uart = unsafe { crate::uart::Uart::new(QEMU_UART_ADDRESS) };
            write!(uart, $fmt, $($e),+).unwrap();
        }
    };
    ($s:literal) => {
        {
            use ::core::fmt::Write;
            const QEMU_UART_ADDRESS: usize = 0x10000000;
            let mut uart = unsafe { crate::uart::Uart::new(QEMU_UART_ADDRESS) };
            write!(uart, $s).unwrap();
        }
    };
}

#[cfg(not(qemu))]
macro_rules! kprintln {
    ($fmt:literal, $($e:expr),+) => {};
    ($s:literal) => {};
}
