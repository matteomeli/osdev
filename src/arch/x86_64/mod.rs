pub mod vga;
pub mod cpuio;
pub mod serial;
pub mod pic;
pub mod interrupts;

#[macro_use]
mod bitflags;

mod irq;
mod descriptor_tables;
