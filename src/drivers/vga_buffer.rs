/*
#[repr(u8)]
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
    LightBrown = 14,
    White = 15
}

pub fn fb_write_cell(i: u32, c: u8, fg: u8, bg: u8) {
    let fb: *mut u8 = 0x000B8000 as *mut _;
    *(fb + i) = c;
    *(fb + i + 1) = ((fg & 0x0F) << 4) | (bg & 0x0F);
}
*/