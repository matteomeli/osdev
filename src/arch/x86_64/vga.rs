//! Basic VGA framebuffer driver.
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
    row: usize,
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

    /// Write the string `s` to screen.
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
            b'\t' => {
                self.col = (self.col + 8) & !(8 - 1);
            }
            // Backspace
            0x08 => {
                if self.col > 0 {
                    self.col -= 1;
                    self.buffer()[self.row][self.col] = Char {
                        code: b' ',
                        colors: self.colors,
                    };
                } else {
                    if self.row > 0 {
                        self.row -= 1;
                        
                        let row = self.row;
                        let mut col = WIDTH - 1;
                        loop {
                            if col == 0 {
                                self.col = col;
                                break;
                            }

                            if self.buffer()[row][col].code != b' ' {
                                self.col = col + 1;
                                break;
                            }

                            col -= 1;
                        }
                    }
                }
            },
            byte => {
                if self.col >= WIDTH {
                    self.new_line();
                }

                let row = self.row;
                let col = self.col;

                self.buffer()[row][col] = Char {
                    code: byte,
                    colors: self.colors,
                };
                self.col += 1;
            }
        }

        // Set cursor
        CURSOR.lock().set(self.row + 1, self.col);
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

        self.col = 0;

        let row = self.row;
        self.clear_row(row);
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
    command: Port<u8>,
    data: Port<u8>,
}

impl Cursor {
    /// Enable the cursor.
    pub fn enable(&mut self) {
        self.command.write(0x0A);
        let dc = self.data.read() & 0x1F;
        self.command.write(0x0A);
        self.data.write(dc & !(0x20));
    }

    /// Set the cursor at the specific row and column.
    pub fn set(&mut self, row: usize, col: usize) {
        let position: usize = (row * WIDTH) + col;

        self.command.write(0x0F);
        self.data.write(position as u8 & 0xFF);
        self.command.write(0x0E);
        self.data.write(((position >> 8) as u8) & 0xFF);
    }
}

pub static SCREEN: Mutex<Screen> = Mutex::new(Screen {
    row: HEIGHT - 2,
    col: 0,
    colors: ColorCode::new(Color::White, Color::Black),
    buffer: unsafe { Unique::new(0xb8000 as *mut _) },
});

pub static CURSOR: Mutex<Cursor> = Mutex::new(Cursor {
    command: Port::new(0x3D4),
    data: Port::new(0x3D5)
});
