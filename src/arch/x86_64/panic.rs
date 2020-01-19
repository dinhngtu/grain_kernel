use core::panic::PanicInfo;

pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
