use core::fmt::Write;
use spin;

unsafe fn outb(port: u16, val: u8) {
    asm!("out dx, al" :: "{al}"(val), "{dx}"(port) :: "intel","volatile");
}

unsafe fn inb(port: u16) -> u8 {
    let mut val: u8;
    asm!("in al, dx" : "={al}"(val) : "{dx}"(port) :: "intel","volatile");
    return val;
}

const SER0: u16 = 0x3f8;

pub struct SerialPort {
    io_port: u16,
}

impl SerialPort {
    pub unsafe fn init(port: u16) -> Self {
        outb(port + 1, 0x00);
        outb(port + 3, 0x80);
        outb(port + 0, 0x03);
        outb(port + 1, 0x00);
        outb(port + 3, 0x03);
        outb(port + 2, 0xC7);
        outb(port + 4, 0x0B);
        SerialPort { io_port: port }
    }
}

impl Write for SerialPort {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        for c in s.as_bytes() {
            unsafe {
                while inb(self.io_port + 5) & 0x20 == 0 {}
                outb(self.io_port, *c);
            }
        }

        Ok(())
    }
}

lazy_static! {
    pub static ref COM1: spin::Mutex<SerialPort> =
        spin::Mutex::new(unsafe { SerialPort::init(SER0) });
}
