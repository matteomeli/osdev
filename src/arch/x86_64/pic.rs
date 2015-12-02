//! The implementation for a 8259 Programmable Interrupt Controller (PIC),
//! which handles I/O interrupts. Eventually, for multicore systems, this
//! implementation will have to be replaces with a APIC interface.
//!
//! This implementation follows the IBM PC 8925 PIC architecture where 2 PICs
//! are used. A slave PIC (PIC2) is chained on the master PIC (PIC1) on IRQ
//! line 2. This allows 15 interrupts instead of only 8.
//!
//! An important note is the mapping of the offset of the two PICs.
//! In protected mode, the IRQ0-7 of PIC1 start at offset 0x08 to 0x0F and
//! IRQ8-15 of PIC2 follow at 0x70 to 0x77. There are 3 classes of interrupts:
//! exceptions (generated internally by the CPU), IRQ or Hardware Interrupt 
//! (managed by the PIC) and software interrutps (coming from programs, usually
//! system calls).
//!
//! The problem here is that, because of a bug in the IBM design, 
//! CPU exceptions are reserved by Intel from 0x00 to 0x1F, 
//! so they would conflict with the IRQ managed by the PIC.
//! Therefore we move PC1 to offset 0x20 to 0x2F and PIC2 to 0x28-0x2F.

use arch::cpuio::{Port, UnsafePort};

/// Command to initialise the PIC
const CMD_INIT: u8 = 0x11;

/// Command to signal end of interrupt 
const CMD_END_OF_INTERRUPT: u8 = 0x20;

/// The mode we want for the PIC configuration
const MODE_8086: u8 = 0x01;

/// Single PIC chip wrapper. Not used individually, hence not accassible.
struct Pic {
    offset: u8,
    command: UnsafePort<u8>,
    data: UnsafePort<u8>,
}

impl Pic {
    /// Signal that an interrupt is being handled.
    unsafe fn end_of_interrupt(&mut self) {
        self.command.write(CMD_END_OF_INTERRUPT);
    }

    /// Check if `interrupt_id` is handled. A PIC handles 8 interrupts.
    fn handles_interrupt(&self, interrupt_id: u8) -> bool {
        interrupt_id >= self.offset && interrupt_id < self.offset + 8
    }

    /// Sets a mask for the Interrupt Mask register to ignore specific interrupts.
    unsafe fn set_mask(&mut self, interrupt_id: u8) {
        let value = self.data.read() | (1 << interrupt_id);
        self.data.write(value);
    }

    unsafe fn clear_mask(&mut self, interrupt_id: u8) {
        let value = self.data.read() & !(1 << interrupt_id);
        self.data.write(value);
    }
}

/// A pair of chained Pics. Standard way on modern x86 architecture.
pub struct ChainedPics {
    master: Pic,
    slave: Pic
}

impl ChainedPics {
    pub const unsafe fn new(offset_master: u8, offset_slave: u8) -> Self {
        ChainedPics {
            master: Pic {
                offset: offset_master,
                command: UnsafePort::new(0x20),
                data: UnsafePort::new(0x21),
            },
            slave: Pic {
                offset: offset_slave,
                command: UnsafePort::new(0xA0),
                data: UnsafePort::new(0xA1),
            },
        }
    }

    /// Initialise both chained PICs together. 
    pub unsafe fn init(&mut self) {
        // Need to wait between sending configuration data to the PICs,
        // especially on old hardware as it could take some time to the
        // PICs to reconfigure. Usually this would be done with a sleep
        // or a delay or something. Probelm is that for those a clock
        // would be needed, but there is no clock yet because a clock
        // needs interrupts!
        // To stop this deadlock, we write dummy data to port 0x80,
        // as it takes long enough on most hardware to have the PIC ready again.
        // Apparently that's safe.
        let mut wait_port: Port<u8> = Port::new(0x80);
        let mut wait = || { wait_port.write(0) };

        let saved_master_mask = self.master.data.read();
        let saved_slave_mask = self.slave.data.read();

        // Send init command
        self.master.command.write(CMD_INIT);
        wait();
        self.slave.command.write(CMD_INIT);
        wait();

        // Setup required offset
        self.master.data.write(self.master.offset);
        wait();
        self.slave.data.write(self.slave.offset);
        wait();

        // Setup master-slave relationship
        self.master.data.write(4);
        wait();
        self.slave.data.write(2);
        wait();

        // Setup mode
        self.master.data.write(MODE_8086);
        wait();
        self.slave.data.write(MODE_8086);
        wait();

        // Restore saved masks
        self.master.data.write(saved_master_mask);
        self.slave.data.write(saved_slave_mask);
    }

    /// Check if `interrupt_id` is handled.
    pub fn handles_interrupt(&self, interrupt_id: u8) -> bool {
        self.master.handles_interrupt(interrupt_id) || 
            self.slave.handles_interrupt(interrupt_id)
    }

    /// Select which PIC needs to know about this interrupt handling chaining.
    pub unsafe fn end_of_interrupt(&mut self, interrupt_id: u8) {
        if self.handles_interrupt(interrupt_id) {
            if self.master.handles_interrupt(interrupt_id) {
                self.master.end_of_interrupt();
            }
            self.slave.end_of_interrupt();
        }
    }

    pub unsafe fn set_mask(&mut self, interrupt_id: u8) {
        if self.handles_interrupt(interrupt_id) {
            if self.master.handles_interrupt(interrupt_id) {
                self.master.set_mask(interrupt_id);
            }
            self.slave.set_mask(interrupt_id);
        }
    }
}

