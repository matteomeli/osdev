///! Interrupt Request Codes descriptions and setup

use core::fmt::{Display, Formatter, Result};

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
    InterruptInfo { id: 0, has_error_code: false, mnemonic: "#DE", description: "Divide Error", irqtype: "Fault", source: "DIV and IDIV instructions." },
    InterruptInfo { id: 1, has_error_code: false, mnemonic: "#DB", description: "RESERVED (Debug)", irqtype: "Fault/Trap", source: "For Intel use only" },
    InterruptInfo { id: 2, has_error_code: false, mnemonic: "#NMI", description: "NMI Interrupt", irqtype: "Interrupt", source: "Nonmaskable external
interrupt." },
    InterruptInfo { id: 3, has_error_code: false, mnemonic: "#BP", description: "Breakpoint", irqtype: "Trap", source: "INT 3 Instruction" },
    InterruptInfo { id: 4, has_error_code: false, mnemonic: "#OF", description: "Overflow", irqtype: "Fault", source: "Divide Error" },
    // TODO
    InterruptInfo { id: 5, has_error_code: false, mnemonic: "#DE", description: "DIV and IDIV instructions.", irqtype: "Divide Error", source: "Fault" },
    InterruptInfo { id: 6, has_error_code: false, mnemonic: "#DE", description: "DIV and IDIV instructions.", irqtype: "Divide Error", source: "Fault" },
    InterruptInfo { id: 7, has_error_code: false, mnemonic: "#DE", description: "DIV and IDIV instructions.", irqtype: "Divide Error", source: "Fault" },
    InterruptInfo { id: 8, has_error_code: false, mnemonic: "#DE", description: "DIV and IDIV instructions.", irqtype: "Divide Error", source: "Fault" },
    InterruptInfo { id: 9, has_error_code: false, mnemonic: "#DE", description: "DIV and IDIV instructions.", irqtype: "Divide Error", source: "Fault" },
    InterruptInfo { id: 10, has_error_code: false, mnemonic: "#DE", description: "DIV and IDIV instructions.", irqtype: "Divide Error", source: "Fault" },
    InterruptInfo { id: 11, has_error_code: false, mnemonic: "#DE", description: "DIV and IDIV instructions.", irqtype: "Divide Error", source: "Fault" },
    InterruptInfo { id: 12, has_error_code: false, mnemonic: "#DE", description: "DIV and IDIV instructions.", irqtype: "Divide Error", source: "Fault" },
    InterruptInfo { id: 13, has_error_code: false, mnemonic: "#DE", description: "DIV and IDIV instructions.", irqtype: "Divide Error", source: "Fault" },
    InterruptInfo { id: 14, has_error_code: false, mnemonic: "#DE", description: "DIV and IDIV instructions.", irqtype: "Divide Error", source: "Fault" },
    InterruptInfo { id: 15, has_error_code: false, mnemonic: "#DE", description: "DIV and IDIV instructions.", irqtype: "Divide Error", source: "Fault" },
    InterruptInfo { id: 16, has_error_code: false, mnemonic: "#DE", description: "DIV and IDIV instructions.", irqtype: "Divide Error", source: "Fault" },
    InterruptInfo { id: 17, has_error_code: false, mnemonic: "#DE", description: "DIV and IDIV instructions.", irqtype: "Divide Error", source: "Fault" },
    InterruptInfo { id: 18, has_error_code: false, mnemonic: "#DE", description: "DIV and IDIV instructions.", irqtype: "Divide Error", source: "Fault" },
    InterruptInfo { id: 19, has_error_code: false, mnemonic: "#DE", description: "DIV and IDIV instructions.", irqtype: "Divide Error", source: "Fault" },
];