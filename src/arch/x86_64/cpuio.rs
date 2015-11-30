//! Wrappers around x86 I/O instructions.

use core::marker::PhantomData;

/// Write `u8`-sized `data` to `port`.
pub unsafe fn outb(port: u16, data: u8) {
    asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(data) :: "volatile");
}

// Read `u8`-sized `data` from `port`.
pub unsafe fn inb(port: u16) -> u8 {
    let result: u8;
    asm!("inb %dx, %al" : "={al}"(result) : "{dx}"(port) :: "volatile");
    result
}

/// Write `u16`-sized `data` to `port`.
pub unsafe fn outw(port: u16, data: u16) {
    asm!("outw %ax, %dx" :: "{dx}"(port), "{ax}"(data) :: "volatile");
}

// Read `u16`-sized `data` from `port`.
pub unsafe fn inw(port: u16) -> u16 {
    let result: u16;
    asm!("inw %dx, %ax" : "={ax}"(result) : "{dx}"(port) :: "volatile");
    result
}

/// Write `u32`-sized `data` to `port`.
pub unsafe fn outl(port: u16, data: u32) {
    asm!("outl %eax, %dx" :: "{dx}"(port), "{eax}"(data) :: "volatile");
}

// Read u32-sized `data` from `port`.
pub unsafe fn inl(port: u16) -> u32 {
    let result: u32;
    asm!("inl %dx, %eax" : "={eax}"(result) : "{dx}"(port) :: "volatile");
    result
}

pub trait InOut {
    /// Write data to the specified port.
    unsafe fn port_out(port: u16, data: Self);

    /// Read data from the specified port.
    unsafe fn port_in(port: u16) -> Self;
}

impl InOut for u8 {
    unsafe fn port_out(port: u16, data: u8) { outb(port, data); }
    unsafe fn port_in(port: u16) -> u8 { inb(port) }
}

impl InOut for u16 {
    unsafe fn port_out(port: u16, data: u16) { outw(port, data); }
    unsafe fn port_in(port: u16) -> u16 { inw(port) }
}

impl InOut for u32 {
    unsafe fn port_out(port: u16, data: u32) { outl(port, data); }
    unsafe fn port_in(port: u16) -> u32 { inl(port) }
}

/// A wrapper for an I/O port supporting an arbitrary type T 
/// implementing `InOut` interface.
///
/// This version of `Port` has safe `read` and `write` functions,
/// so it's appropriate to use with hardware that cannot violate
/// safety guarantees, i.e. COM1 serial port.
#[derive(Debug)]
pub struct Port<T: InOut> {
    port: u16,

    // zero-byte placeholder to have parameter type T without compiler errors
    phantom: PhantomData<T>,
}

impl<T: InOut> Port<T> {
    /// Create a new I/O port.
    pub const fn new(port: u16) -> Self {
        Port { port: port, phantom: PhantomData }
    }

    /// Write data to the port.
    pub fn write(&mut self, data: T) {
        unsafe { T::port_out(self.port, data); }
    }

    /// Read data from the port.
    pub fn read(&mut self) -> T {
        unsafe { T::port_in(self.port) }
    }
}

/// An unsafe wrapper to I/O port supporting an arbitrary type T 
/// implementing the `InOut` interface.
///
/// This version of `Port` has unsafe `read` and `write` functions,
/// and it's appropriate to use with hardware that can potentially
/// cause undefinied behaviour or corrupt memory, i.e interrupt controller.
pub struct UnsafePort<T: InOut> {
    port: u16,
    phantom: PhantomData<T>,
}

impl<T: InOut> UnsafePort<T> {
    /// Create a new I/O unsafe port.
    pub const unsafe fn new(port: u16) -> Self {
        UnsafePort { port: port, phantom: PhantomData }
    }

    /// Read data from the port.
    pub unsafe fn read(&mut self) -> T {
        T::port_in(self.port)
    }

    /// Write data to the port.
    pub unsafe fn write(&mut self, data: T) {
        T::port_out(self.port, data);
    }
}
