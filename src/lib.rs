// https://doc.rust-lang.org/book/no-stdlib.html

#![feature(no_std, lang_items, core_str_ext, const_fn, unique, asm)]
#![no_std]

extern crate rlibc;
extern crate spin;

#[macro_use]
mod macros;
mod arch;
mod console;

#[no_mangle] // ensure that this symbol is called `main` in the output
pub extern "C" fn rust_main() {
    use arch::vga::{SCREEN, CURSOR, ColorCode};
    use arch::vga::Color::*;

    CURSOR.lock().enable();
    SCREEN.lock()
        .set_colors(ColorCode::new(White, Black))
        .clear();
    println!("Hello World!");

    loop {}
}

// These functions and traits are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
#[lang = "panic_fmt"]
extern "C" fn panic_fmt() -> ! {
    loop {}
}
