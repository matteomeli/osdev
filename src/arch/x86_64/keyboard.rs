//! A PS/2 keyboard driver

use arch::cpuio::Port;
use spin::Mutex;

#[derive(Debug)]
struct Modifiers {
    left_shift: bool,
    right_shift: bool,
    left_ctrl: bool,
    right_ctrl: bool,
    left_alt: bool,
    right_alt: bool,
    caps_lock: bool,
}

impl Modifiers {
    const fn new() -> Self {
        Modifiers {
            left_shift: false,
            right_shift: false,
            left_ctrl: false,
            right_ctrl: false,
            left_alt: false,
            right_alt: false,
            caps_lock: false,
        }
    }

    fn apply(&self, ascii: u8) -> u8 {
        if ascii >= b'a' && ascii <= b'z' {
            if self.use_uppercase() {
                return ascii - b'a' + b'A';
            }
        }
        ascii
    }

    fn update(&mut self, scancode: u8) {
        match scancode {
            0x2A => self.left_shift = true,
            0x36 => self.right_shift = true,
            0x1D => self.left_ctrl = true,
            0x38 => self.left_alt = true,

            0x3A => self.caps_lock = !self.caps_lock,

            0xAA => self.left_shift = false,
            0xB6 => self.right_shift = false,
            0x9D => self.left_ctrl = false,
            0xB8 => self.left_alt = false,

            _ => {}
        }
    }

    fn use_uppercase(&self) -> bool {
        self.left_shift || self.right_shift || self.caps_lock
    }
}

pub struct KeyboardState {
    port: Port<u8>,
    modifiers: Modifiers,
}

impl KeyboardState {
    pub fn get_char(&mut self) -> Option<char> {
        let scancode = self.port.read();

        self.modifiers.update(scancode);

        if let Some(ascii) = self.find_ascii(scancode) {
            Some(self.modifiers.apply(ascii) as char)
        } else {
            None
        }
    }

    fn find_ascii(&self, scancode: u8) -> Option<u8> {
        let index = scancode as usize;
        match scancode {
            0x01 ... 0x0E => Some(b"\x1B1234567890-=\x08"[index-0x01]),
            0x0F ... 0x1C => Some(b"\tqwertyuiop[]\r"[index-0x0F]),
            0x1E ... 0x28 => Some(b"asdfghjkl;'"[index-0x1E]),
            0x2C ... 0x35 => Some(b"zxcvbnm,./"[index-0x2C]),
            0x39 => Some(b' '),
            _ => None
        }
    }
}

pub static STATE: Mutex<KeyboardState> = Mutex::new(KeyboardState {
    port: Port::new(0x60),
    modifiers: Modifiers::new()
});
