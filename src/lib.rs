#![no_std]
#![feature(asm)]

mod arch;
mod panic;

#[no_mangle]
pub extern "C" fn start() -> ! {
    //panic!("start");
    arch::print::print_init();
    arch::print::raw_print("hello world");
    panic!();
}
