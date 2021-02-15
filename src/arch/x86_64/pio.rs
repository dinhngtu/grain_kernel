pub trait PortIO {
    unsafe fn port_out(port: u16, val: Self);
    unsafe fn port_in(port: u16) -> Self;
}

#[inline(always)]
pub unsafe fn outb(port: u16, val: u8) {
    asm!("out dx, al", in("al") val, in("dx") port)
}

#[inline(always)]
pub unsafe fn inb(port: u16) -> u8 {
    let mut val: u8;
    asm!("in al, dx", out("al") val, in("dx") port);
    return val;
}

#[inline(always)]
pub unsafe fn outw(port: u16, val: u16) {
    asm!("out dx, ax", in("ax") val, in("dx") port)
}

#[inline(always)]
pub unsafe fn inw(port: u16) -> u16 {
    let mut val: u16;
    asm!("in ax, dx", out("ax") val, in("dx") port);
    return val;
}

#[inline(always)]
pub unsafe fn outl(port: u16, val: u32) {
    asm!("out dx, eax", in("eax") val, in("dx") port)
}

#[inline(always)]
pub unsafe fn inl(port: u16) -> u32 {
    let mut val: u32;
    asm!("in eax, dx", out("eax") val, in("dx") port);
    return val;
}

#[inline(always)]
pub unsafe fn iodelay() {
    outb(0xed, 0);
}
