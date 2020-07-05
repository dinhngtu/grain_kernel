use crate::arch::serial::COM1;
use core::fmt::Write;
use core::panic::PanicInfo;

pub fn panic(info: &PanicInfo) -> ! {
    writeln!(*COM1.lock(), "Panic: {}", info).unwrap();
    loop {
        unsafe {
            asm!("cli", "hlt");
        }
    }
}
