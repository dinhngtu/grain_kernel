use crate::arch::serial::{SerialPort, SER0};
use core::fmt::Write;
use core::panic::PanicInfo;

pub fn panic(info: &PanicInfo) -> ! {
    let mut com1 = unsafe { SerialPort::create(SER0) };
    {
        writeln!(com1, "Panic: {}", info);
    }
    loop {
        unsafe {
            asm!(r"
                cli
                hlt
                " :::: "intel");
        }
    }
}
