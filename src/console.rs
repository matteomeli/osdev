//! A wrapper around a VGA console and a COM1 serial port.serial

use core::fmt::{Write, Result};
use spin::Mutex;
use arch::{vga, serial};

pub struct Console;

impl Write for Console {
    fn write_str(&mut self, s: &str) -> Result {
        try!(vga::SCREEN.lock().write_str(s));
        serial::COM1.lock().write_str(s)
    }
}

pub static CONSOLE: Mutex<Console> = Mutex::new(Console);
