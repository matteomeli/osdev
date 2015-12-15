// https://doc.rust-lang.org/book/no-stdlib.html

#![feature(no_std, lang_items, core_str_ext, core_slice_ext, const_fn, unique, asm, iter_cmp)]
#![no_std]

extern crate rlibc;
extern crate spin;
extern crate multiboot2;

pub use arch::interrupts::rust_interrupt_handler;

#[macro_use]
mod bitflags;

#[macro_use]
mod macros;
mod arch;
mod console;
mod memory;

mod std {
    pub use core::fmt;
    pub use core::ops;
    pub use core::option;
}

#[no_mangle] // ensure that this symbol is called `main` in the output
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    use arch::vga::{SCREEN, CURSOR, ColorCode};
    use arch::vga::Color::*;

    CURSOR.lock().enable();
    SCREEN.lock()
        .set_colors(ColorCode::new(White, Black))
        .clear();
    println!("Hello World!");

    unsafe {
        arch::interrupts::init();
    }

    println!("Running...");

    println!("");

    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };
    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");

    println!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!("\tstart: {:#x}, length: {:#x}", area.base_addr, area.length);
    }

    let elf_sections_tag = boot_info.elf_sections_tag()
        .expect("Elf-sections tag required");

    println!("kernel sections:");
    for section in elf_sections_tag.sections() {
        println!("\taddr: {:#x}, size: {:#x}, flags: {:#x}",
            section.addr, section.size, section.flags);
    }

    let kernel_start = elf_sections_tag.sections().map(|s| s.addr)
        .min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size)
        .max().unwrap();

    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);

    println!("kernel_start: {:#x}, kernel_end: {:#x}", kernel_start, kernel_end);
    println!("multiboot_start: {:#x}, multiboot_end: {:#x}", multiboot_start, multiboot_end);

    let mut frame_allocator = memory::AreaFrameAllocator::new(
        kernel_start as usize, kernel_end as usize, 
        multiboot_start as usize, multiboot_end as usize, memory_map_tag.memory_areas());

    memory::test_paging(&mut frame_allocator);

    loop {}
}

// These functions and traits are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt(fmt: core::fmt::Arguments, file: &str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("\t{}", fmt);
    loop {}
}
