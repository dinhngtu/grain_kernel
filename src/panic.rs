use crate::arch;
use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    arch::panic::panic(info);
}
