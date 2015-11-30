
// TODO

const VGA_TEXTBUFFER_COMMAND_PORT: u16 = 0x3D4;
const VGA_TEXTBUFFER_DATA_PORT: u16 = 0x3D5;

const VGA_TEXTBUFFER_HB_COMMAND: u8 = 14;
const VGA_TEXTBUFFER_LB_COMMAND: u8 = 15;

/// TODO: Write comment
pub fn outb(port: u16, data: u8) {
    unsafe {
        asm!("out %dx, %al" :: 
            "{dx}"(port), "{al}"(data) :: 
            "volatile");
    }
}

pub fn fb_move_cursor(position: u16) {
    outb(VGA_TEXTBUFFER_COMMAND_PORT, VGA_TEXTBUFFER_HB_COMMAND);
    outb(VGA_TEXTBUFFER_DATA_PORT, ((position >> 8) as u8) & 0x00FF);
    outb(VGA_TEXTBUFFER_COMMAND_PORT, VGA_TEXTBUFFER_HB_COMMAND);
    outb(VGA_TEXTBUFFER_DATA_PORT, (position & 0x00FF) as u8);
}

pub fn write(s: &str) {
    // TODO
}