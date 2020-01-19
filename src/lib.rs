#![no_std]

mod arch;
mod panic;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    panic!("start");
}
