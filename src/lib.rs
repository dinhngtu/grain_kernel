#![no_std]
#![feature(asm)]
#![feature(never_type)]
#![feature(optin_builtin_traits)]

#[macro_use]
extern crate lazy_static;

mod arch;
mod panic;
mod util;

use crate::arch::serial::COM1;
use core::fmt::Write;

pub fn start() -> ! {
    writeln!(*COM1.lock(), "hello world").unwrap();
    panic!("nothing to do, dying");
}
