//! A wrapper around the VGA framebuffer.
// Based on http://os.phil-opp.com/printing-to-screen.html

use core::fmt::{Write, Result};
use core::ptr::Unique;
use spin::Mutex;
use arch::cpuio::Port;

const HEIGHT: usize = 25;
const WIDTH: usize = 80;

/// Standard VGA colors.
#[repr(u8)]
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGrey = 7,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    Yellow = 14,
    White = 15,
}

/// VGA compound color codes.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ColorCode(u8);

impl ColorCode {
    pub const fn new(foreground: Color, background: Color) -> Self {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

/// A coloured VGA character.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Char {
    code: u8,
    colors: ColorCode,
}

type Buffer = [[Char; WIDTH]; HEIGHT];

/// A VGA screen in character mode.
pub struct Screen {
    col: usize,
    colors: ColorCode,
    buffer: Unique<Buffer>,
}

impl Screen {
    /// Clear the screen.
    pub fn clear(&mut self) -> &mut Self {
        let c = Char {
            code: b' ',
            colors: self.colors,
        };
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                self.buffer()[row][col] = c;
            }
        }
        self
    }

    /// Set current text colors.
    pub fn set_colors(&mut self, colors: ColorCode) -> &mut Self {
        self.colors = colors;
        self
    }

    /// Write the string `s` to screen
    pub fn write(&mut self, s: &str) {
        self.write_bytes(s.as_bytes())
    }

    /// Write the `u8`-sized character array to screen.
    pub fn write_bytes(&mut self, text: &[u8]) {
        for c in text {
            self.write_byte(*c);
        }
    }

    /// Write a single byte to the screen.
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.col >= WIDTH {
                    self.new_line();
                }

                let row = HEIGHT - 2;
                let col = self.col;

                self.buffer()[row][col] = Char {
                    code: byte,
                    colors: self.colors,
                };
                self.col += 1;

                CURSOR.lock().set(HEIGHT - 1, self.col);
            }
        }
    }

    fn buffer(&mut self) -> &mut Buffer {
        unsafe { self.buffer.get_mut() }
    }

    fn new_line(&mut self) {
        {
            let buffer = self.buffer();
            for row in 0..(HEIGHT - 1) {
                buffer[row] = buffer[row + 1];
            }
        }
        self.clear_row(HEIGHT - 1);
        self.col = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = Char {
            code: b' ',
            colors: self.colors,
        };
        self.buffer()[row] = [blank; WIDTH];
    }
}

impl Write for Screen {
    fn write_str(&mut self, s: &str) -> Result {
        self.write(s);
        Ok(())
    }
}

/// A VGA screen cursor.
pub struct Cursor {
    command_port: Port<u8>,
    data_port: Port<u8>,
}

impl Cursor {
    /// Enable the cursor.
    pub fn enable(&mut self) {
        self.command_port.write(0x0A);
        let dc = self.data_port.read() & 0x1F;
        self.command_port.write(0x0A);
        self.data_port.write(dc & !(0x20));
    }

    /// Set the cursor at the specific row and column.
    pub fn set(&mut self, row: usize, col: usize) {
        let position: usize = (row * WIDTH) + col;

        self.command_port.write(0x0F);
        self.data_port.write(position as u8 & 0xFF);
        self.command_port.write(0x0E);
        self.data_port.write(((position >> 8) as u8) & 0xFF);
    }
}

pub static SCREEN: Mutex<Screen> = Mutex::new(Screen {
    col: 0,
    colors: ColorCode::new(Color::White, Color::Black),
    buffer: unsafe { Unique::new(0xb8000 as *mut _) },
});

pub static CURSOR: Mutex<Cursor> = Mutex::new(Cursor {
    command_port: unsafe { Port::new(0x3D4) },
    data_port: unsafe { Port::new(0x3D5) }
});
