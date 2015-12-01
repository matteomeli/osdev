//! Basic COM serial port driver.
// http://wiki.osdev.org/Serial_ports

use core::fmt::{Write, Result};
use arch::cpuio::UnsafePort;
use self::SerialRegister::*;
use spin::Mutex;

/// Each COM serial port has 8 data registers, offset from the port address.
/// The first two have dual use depending on DLAB bit in the `LineControl` register.
#[repr(C, u8)]
#[allow(dead_code)]
enum SerialRegister {
    DataOrBaudDivisorLowByte = 0,
    InterruptEnableOrBaudDivisorHighByte = 1,
    InterruptIdentificationAndFifo = 2,
    LineControl = 3,
    ModemControl = 4,
    LineStatus = 5,
    ModemStatus = 6,
    Scratch = 7,
}

/// A COM serial port wrapper.
pub struct SerialPort {
    base_address: u16
}

impl SerialPort {
    const unsafe fn new(base_address: u16) -> Self {
        SerialPort { base_address: base_address }
    }

    unsafe fn lazy_init(&self) {
        // Disable all interrupts
        self.port(InterruptEnableOrBaudDivisorHighByte).write(0x00);

        // Enable DLAB (set baud rate divisor)
        let saved_line_control_mode = self.port(LineControl).read();
        self.port(LineControl).write(0x80 | saved_line_control_mode);

        // Set divisor to 3
        let baud_divisor: u16 = 2;
        self.port(DataOrBaudDivisorLowByte).write(baud_divisor as u8);
        self.port(InterruptEnableOrBaudDivisorHighByte).write((baud_divisor >> 8) as u8);

        // Restore mode on LineControl register
        self.port(LineControl).write(saved_line_control_mode);

        // Set 8N1 mode (8 bits, no parity, one stop bit)
        self.port(LineControl).write(0x03);

        // Enable FIFO, clear them, with 14 byte threshold
        self.port(InterruptIdentificationAndFifo).write(0xC7);

        // Configure modem: IRQs enabled, RTS/DSR on
        self.port(ModemControl).write(0x0B);
    }

    /// Get an `UnsafePort` instead of a `Port` because
    /// the returned port could be potentially use to mess with
    /// the CPU interrupts. 
    /// I.e, t's not safe as the simple VGA text cursor port.
    unsafe fn port(&self, reg: SerialRegister) -> UnsafePort<u8> {
        UnsafePort::new(self.base_address + (reg as u8 as u16))
    }

    fn can_transmit(&self) -> bool {
        unsafe { self.port(LineStatus).read() & 0x20 != 0 }
    }
}

impl Write for SerialPort {
    fn write_str(&mut self, s: &str) -> Result {
        unsafe {
            self.lazy_init();

            for &b in s.as_bytes() {
                while !self.can_transmit() {}

                self.port(DataOrBaudDivisorLowByte).write(b);
            }
        }
        Ok(())
    }
}

/// The COM1 port
pub static COM1: Mutex<SerialPort> = Mutex::new(unsafe {
    SerialPort::new(0x03F8)
});
