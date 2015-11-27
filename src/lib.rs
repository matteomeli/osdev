// https://doc.rust-lang.org/book/no-stdlib.html

#![feature(no_std, lang_items)]
#![no_std]

extern crate rlibc;

mod drivers;
use drivers::vga_buffer::*;

#[no_mangle] // ensure that this symbol is called `main` in the output
pub extern fn rust_main() {
    //let x = ["Hello", "", "World", "!"];
    //let test = (0..3).flat_map(|x| 0..3).zip(0..);
    //let mut a = 42;
    //a += 1;

    let hello = b"Hello World!";
    let color_byte = 0x1f; // white foreground, blue background

    let mut hello_colored = [color_byte; 24];
    for (i, char_byte) in hello.into_iter().enumerate() {
        hello_colored[i*2] = *char_byte;
    }

    // write `Hello World!` to the center of the VGA text buffer
    let buffer_ptr = (0xb8000 + 1988) as *mut _;
    unsafe { *buffer_ptr = hello_colored };

    fb_write_cell(0, 'A', 2, 8);

    loop{}
}

// These functions and traits are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop{} }