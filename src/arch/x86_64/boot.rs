//use crate::arch::multiboot2::*;
use crate::arch::serial::{SerialPort, SER0};
use core::ffi::c_void;

#[no_mangle]
pub extern "sysv64" fn x86_64_start(ptr: *const c_void) -> ! {
    {
        unsafe {
            SerialPort::init(SER0);
        }
    }

    crate::start();
}
