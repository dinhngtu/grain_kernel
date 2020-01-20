use crate::arch::multiboot2::*;
use crate::arch::serial::COM1;
use core::ffi::c_void;
use core::fmt::Write;

#[no_mangle]
pub extern "sysv64" fn x86_64_start(ptr: *const c_void) -> ! {
    for tag in BootInfoReader::from(ptr) {
        match tag {
            BootInfoTag::Cmdline(s) => writeln!(*COM1.lock(), "Command line: {}", s).unwrap(),
            BootInfoTag::Unknown => {}
            _ => {}
        };
    }

    crate::start();
}
