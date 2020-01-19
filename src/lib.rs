#![no_std]

mod arch;
mod panic;

#[no_mangle]
pub extern "C" fn start() -> ! {
    panic!("start");
}

#[no_mangle]
pub extern "C" fn multiboot2_start() -> ! {
    panic!("multiboot start");
}
