use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

const COM1: u16 = 0x3F8;

fn outb(port: u16, val: u8) {
    unsafe {
        core::arch::asm!("out dx, al", in("dx") port, in("al") val, options(nomem, nostack));
    }
}

fn inb(port: u16) -> u8 {
    let val: u8;
    unsafe {
        core::arch::asm!("in al, dx", out("al") val, in("dx") port, options(nomem, nostack));
    }
    val
}

pub struct SerialPort {
    port: u16,
}

impl SerialPort {
    pub fn new(port: u16) -> Self {
        // Initialize the serial port
        outb(port + 1, 0x00); // Disable all interrupts
        outb(port + 3, 0x80); // Enable DLAB (set baud rate divisor)
        outb(port + 0, 0x01); // Set divisor to 1 (115200 baud)
        outb(port + 1, 0x00); //   (hi byte)
        outb(port + 3, 0x03); // 8 bits, no parity, one stop bit
        outb(port + 2, 0xC7); // Enable FIFO, clear, 14-byte threshold
        outb(port + 4, 0x0B); // IRQs enabled, RTS/DSR set

        SerialPort { port }
    }

    fn is_transmit_empty(&self) -> bool {
        inb(self.port + 5) & 0x20 != 0
    }

    pub fn write_byte(&mut self, byte: u8) {
        while !self.is_transmit_empty() {
            core::hint::spin_loop();
        }
        outb(self.port, byte);
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            if byte == b'\n' {
                self.write_byte(b'\r');
            }
            self.write_byte(byte);
        }
        Ok(())
    }
}

lazy_static! {
    pub static ref SERIAL: Mutex<SerialPort> = Mutex::new(SerialPort::new(COM1));
}
