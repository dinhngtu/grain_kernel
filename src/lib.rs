#![no_std]
#![feature(asm)]
#![feature(never_type)]

mod arch;
mod panic;

use crate::arch::serial::{SerialPort, SER0};
use core::fmt::Write;

pub fn start() -> ! {
    let mut com1 = unsafe { SerialPort::create(SER0) };
    {
        writeln!(com1, "hello world");
    }
    panic!("test panic");
}
