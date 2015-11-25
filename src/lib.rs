// https://doc.rust-lang.org/book/no-stdlib.html

#![feature(no_std, lang_items)]
#![no_std]

extern crate rlibc;

mod drivers;

#[no_mangle] // ensure that this symbol is called `main` in the output
pub extern fn rust_main() {
    //let x = ["Hello", "", "World", "!"];
}

// These functions and traits are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop{} }