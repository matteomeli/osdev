//! Data structures and functions to handle descriptor tables: GDT/LDT/IDT.

/// A struct that wraps a pointer to data to represent an IDT
/// and it's suitable to use within the assembly instruction lidt.
#[derive(Debug)]
#[repr(C, packed)]
pub struct DescriptorTablePointer {
    /// The size of the IDT.
    pub limit: u16,
    /// Pointer to the memory region containing the IDT.
    pub base: u64,
}

/// Load the IDT into memory.
pub unsafe fn lidt(idt: &DescriptorTablePointer) {
    asm!("lidt ($0)" :: "r"(idt) : "memory");
}
