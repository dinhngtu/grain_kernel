use crate::arch::pio::{inb, outb};
use core::fmt::Write;
use spin;

const SER0: u16 = 0x3f8;

/// RO
const RBR: u16 = 0;

/// WO
const THR: u16 = 0;

/// DLAB RW
const DLL: u16 = 0;

/// RW
const IER: u16 = 1;
const IER_RDI: u8 = 0x1;
const IER_THRI: u8 = 0x2;
const IER_RLSI: u8 = 0x4;
const IER_MSI: u8 = 0x8;

/// DLAB RW
const DLM: u16 = 1;

/// RO
const IIR: u16 = 2;
const IIR_NOINT: u8 = 0x1;
const IIR_THRI: u8 = 0x2;
const IIR_RDI: u8 = 0x4;
const IIR_RLSI: u8 = 0x6;
const IIR_INTMASK: u8 = 0xe;

/// WO
const FCR: u16 = 2;
const FCR_FIFO: u8 = 0x1;
const FCR_CLEAR_RX: u8 = 0x2;
const FCR_CLEAR_TX: u8 = 0x4;
const FCR_DMA_SELECT: u8 = 0x8;

const LCR: u16 = 3;
const LCR_WLEN5: u8 = 0x0;
const LCR_WLEN6: u8 = 0x1;
const LCR_WLEN7: u8 = 0x2;
const LCR_WLEN8: u8 = 0x3;
const LCR_STOP: u8 = 0x4;
const LCR_PARITY: u8 = 0x8;
const LCR_EPAR: u8 = 0x10;
const LCR_SPAR: u8 = 0x20;
const LCR_SBC: u8 = 0x40;
const LCR_DLAB: u8 = 0x80;

const MCR: u16 = 4;
const MCR_DTR: u8 = 0x1;
const MCR_RTS: u8 = 0x2;
const MCR_OUT1: u8 = 0x4;
const MCR_OUT2: u8 = 0x8;
const MCR_LOOP: u8 = 0x10;
const MCR_AFE: u8 = 0x20;

/// RO
const LSR: u16 = 5;
const LSR_DR: u8 = 0x1;
const LSR_OE: u8 = 0x2;
const LSR_PE: u8 = 0x4;
const LSR_FE: u8 = 0x8;
const LSR_BI: u8 = 0x10;
const LSR_THRE: u8 = 0x20;
const LSR_TEMT: u8 = 0x40;
const LSR_FIFOE: u8 = 0x80;

/// RO
const MSR: u16 = 6;
const MSR_DCTS: u8 = 0x1;
const MSR_DDSR: u8 = 0x2;
const MSR_TERI: u8 = 0x4;
const MSR_DDCD: u8 = 0x8;
const MSR_CTS: u8 = 0x10;
const MSR_DSR: u8 = 0x20;
const MSR_RI: u8 = 0x40;
const MSR_DCD: u8 = 0x80;

const SCR: u16 = 7;

pub struct SerialPort {
    pio_base: u16,
}

impl SerialPort {
    pub unsafe fn init(pio_base: u16, rate: u32) -> Self {
        let mut port = SerialPort { pio_base };
        outb(port.pio_base + IER, 0);
        outb(port.pio_base + LCR, LCR_WLEN8);
        port.set_baudrate(rate);
        port
    }

    pub unsafe fn set_baudrate(&mut self, rate: u32) {
        assert!(rate <= 115200);
        let old_lcr = inb(self.pio_base + LCR);
        let divisor = 115200 / rate;
        outb(self.pio_base + LCR, LCR_DLAB);
        outb(self.pio_base + DLL, (divisor & 0xff) as u8);
        outb(self.pio_base + DLM, ((divisor >> 8) & 0xff) as u8);
        outb(self.pio_base + LCR, old_lcr);
    }
}

impl Write for SerialPort {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        for c in s.as_bytes() {
            unsafe {
                while inb(self.pio_base + LSR) & LSR_THRE == 0 {}
                outb(self.pio_base + THR, *c);
                while inb(self.pio_base + LSR) & LSR_TEMT == 0 {}
            }
        }

        Ok(())
    }
}

impl !Sync for SerialPort {}

lazy_static! {
    pub static ref COM1: spin::Mutex<SerialPort> =
        spin::Mutex::new(unsafe { SerialPort::init(SER0, 115200) });
}
