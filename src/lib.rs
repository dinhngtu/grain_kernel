#![no_std]
#![feature(asm)]
#![feature(never_type)]

#[macro_use]
extern crate lazy_static;

mod arch;
mod panic;

use crate::arch::serial::COM1;
use core::fmt::Write;

pub fn start() -> ! {
    writeln!(*COM1.lock(), "hello world").unwrap();
    panic!("nothing to do, dying");
}
