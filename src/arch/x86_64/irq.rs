///! Interrupt Request Codes descriptions and setup

use core::fmt::{Display, Debug, Formatter, Result};

/// Aggregates some information about an interrupt.
#[derive(Debug)]
pub struct InterruptInfo {
    id: u8,
    has_error_code: bool,
    mnemonic: &'static str,
    description: &'static str,
    irqtype: &'static str,
    source: &'static str,
}

impl Display for InterruptInfo {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} ({}, vec={}) {}", self.mnemonic, self.irqtype, self.id, self.description)
    }
}

/// Processor Exceptions/Interrupts.
///
/// First 32 interrupts are reserved. Only first 20 are actually used.
pub static CPU_EXCEPTIONS: [InterruptInfo; 20] = [
    InterruptInfo { id: 0, has_error_code: false, mnemonic: "#DE", description: "Divide Error", irqtype: "Fault", source: "DIV and IDIV instructions" },
    InterruptInfo { id: 1, has_error_code: false, mnemonic: "#DB", description: "RESERVED (Debug)", irqtype: "Fault/Trap", source: "For Intel use only" },
    InterruptInfo { id: 2, has_error_code: false, mnemonic: "#NMI", description: "NMI Interrupt", irqtype: "Interrupt", source: "Nonmaskable external interrupt" },
    InterruptInfo { id: 3, has_error_code: false, mnemonic: "#BP", description: "Breakpoint", irqtype: "Trap", source: "INT 3 Instruction" },
    InterruptInfo { id: 4, has_error_code: false, mnemonic: "#OF", description: "Overflow", irqtype: "Fault", source: "Divide Error" },
    InterruptInfo { id: 5, has_error_code: false, mnemonic: "#BR", description: "BOUND Range Exceeded", irqtype: "Fault", source: "Bound instruction" },
    InterruptInfo { id: 6, has_error_code: false, mnemonic: "#UD", description: "Invalid Opcode (Undefined Opcode)", irqtype: "Fault", source: "UD2 instruction or reserved opcode" },
    InterruptInfo { id: 7, has_error_code: true, mnemonic: "#NM", description: "Device not accessible (No Math Coprocessor)", irqtype: "Fault", source: "Floating-point or WAIT/FWAIT instruction" },
    InterruptInfo { id: 8, has_error_code: true, mnemonic: "#DF", description: "Double Fault", irqtype: "Abort", source: "Any source that can generate an exception, an NMI, or a NTR" },
    InterruptInfo { id: 9, has_error_code: false, mnemonic: "#CSO", description: "Coprocessor Segment Overrun (reserved)", irqtype: "Fault", source: "Floating-point instruction" },
    InterruptInfo { id: 10, has_error_code: true, mnemonic: "#TS", description: "Invalid TSS", irqtype: "Fault", source: "Task switch or TSS access" },
    InterruptInfo { id: 11, has_error_code: true, mnemonic: "#NP", description: "Segment not present", irqtype: "Fault", source: "Loading segment registers or accessing system segments" },
    InterruptInfo { id: 12, has_error_code: true, mnemonic: "#SS", description: "Stak-Segment fault", irqtype: "Fault", source: "Stack operations and SS register loads" },
    InterruptInfo { id: 13, has_error_code: true, mnemonic: "#GP", description: "General protection", irqtype: "Fault", source: "Any memory reference and other protection checks" },
    InterruptInfo { id: 14, has_error_code: true, mnemonic: "#PF", description: "Page fault", irqtype: "Fault", source: "Any memory reference" },
    InterruptInfo { id: 15, has_error_code: false, mnemonic: "-", description: "Intel reserved (Do not use)", irqtype: "-", source: "-" },
    InterruptInfo { id: 16, has_error_code: false, mnemonic: "#MF", description: "x87 FPU Floating-Point Error (Math Fault)", irqtype: "Fault", source: "x87 FPU floating-point or WAIT/WAITF instruction" },
    InterruptInfo { id: 17, has_error_code: true, mnemonic: "#AC", description: "Alignment Check", irqtype: "Fault", source: "Any data reference in memory" },
    InterruptInfo { id: 18, has_error_code: false, mnemonic: "#MC", description: "Machine Check", irqtype: "Abort", source: "Error codes (if any) and source are model dependent" },
    InterruptInfo { id: 19, has_error_code: false, mnemonic: "#XM", description: "SIMD Floating-Point Exception", irqtype: "Fault", source: "SSE/SSE2/SSE3 floating-point instruction" },
];

bitflags!{
    flags PageFaultException: u32 {
        // 0 The fault was caused by a non-present page.
        // 1 The fault was caused by a page-level protection violation.
        const PAGEFAULT_EXCEPTION_P =  bit!(0),

        // 0 The access causing the fault was a read.
        // 1 The access causing the fault was a write.
        const PAGEFAULT_EXCEPTION_WR = bit!(1),

        // 0 The access causing the fault originated when the processor was executing in supervisor mode (CPL < 3).
        // 1 The access causing the fault originated when the processor was executing in user mode (CPL = 3).
        const PAGEFAULT_EXCEPTION_US = bit!(2),

        // 0 The fault was not caused by reserved bit violation.
        // 1 The fault was caused by a reserved bit set to 1 in some paging-stucture entry.
        const PAGEFAULT_EXCEPTION_RSVD = bit!(3),

        // 0 The fault was not caused by an instruction fetch.
        // 1 The fault was caused by an instruction fetch.
        const PAGEFAULT_EXCEPTION_ID = bit!(4)
    }
}

impl Debug for PageFaultException {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let p = match self.contains(PAGEFAULT_EXCEPTION_P) {
            false => "The fault was caused by a non-present page.",
            true => "The fault was caused by a page-level protection violation."
        };
        let wr = match self.contains(PAGEFAULT_EXCEPTION_WR) {
            false => "The access causing the fault was a read.",
            true => "The access causing the fault was a write."
        };
        let us = match self.contains(PAGEFAULT_EXCEPTION_US) {
            false => "The access causing the fault originated when the processor was executing in supervisor mode (CPL < 3).",
            true => "The access causing the fault originated when the processor was executing in user mode (CPL = 3)."
        };
        let rsvd = match self.contains(PAGEFAULT_EXCEPTION_RSVD) {
            false => "The fault was not caused by reserved bit violation.",
            true => "The fault was caused by a reserved bit set to 1 in some paging-stucture entry."
        };
        let id = match self.contains(PAGEFAULT_EXCEPTION_ID) {
            false => "The fault was not caused by an instruction fetch.",
            true => "The fault was caused by an instruction fetch."
        };
        write!(f, "{}\n{}\n{}\n{}\n{}", p, wr, us, rsvd, id)
    }
}