use crate::arch::multiboot2::*;
use crate::arch::serial::{SerialPort, SER0};
use core::ffi::c_void;
use core::fmt::Write;

#[no_mangle]
pub extern "sysv64" fn x86_64_start(ptr: *const c_void) -> ! {
    let mut com1 = unsafe { SerialPort::init(SER0) };

    for tag in BootInfoReader::from(ptr) {
        match tag {
            BootInfoTag::Cmdline(s) => writeln!(com1, "Command line: {}", s).unwrap(),
            BootInfoTag::Unknown => {}
            _ => {}
        };
    }

    crate::start();
}
