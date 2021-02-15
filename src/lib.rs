#![no_std]
#![feature(asm)]
#![feature(never_type)]
#![feature(negative_impls)]

#[macro_use]
extern crate lazy_static;

mod arch;
mod panic;
mod util;

pub fn start() -> ! {
    panic!("nothing to do, dying");
}
