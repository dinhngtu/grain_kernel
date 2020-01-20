unsafe fn outb(port: u16, val: u8) {
    asm!("out dx, al" :: "{al}"(val), "{dx}"(port) :: "intel","volatile");
}

unsafe fn inb(port: u16) -> u8 {
    let mut val: u8;
    asm!("in al, dx" : "={al}"(val) : "{dx}"(port) :: "intel","volatile");
    return val;
}

const SER0: u16 = 0x3f8;

pub unsafe fn print_init_port(port: u16) {
    outb(port + 1, 0x00);
    outb(port + 3, 0x80);
    outb(port + 0, 0x03);
    outb(port + 1, 0x00);
    outb(port + 3, 0x03);
    outb(port + 2, 0xC7);
    outb(port + 4, 0x0B);
}

pub fn print_init() {
    unsafe {
        print_init_port(SER0);
    }
}

pub unsafe fn raw_print_port(port: u16, s: &str) {
    for c in s.as_bytes() {
        while inb(port + 5) & 0x20 == 0 {}
        outb(port, *c);
    }
}

pub fn raw_print(s: &str) {
    unsafe {
        raw_print_port(SER0, s);
    }
}
