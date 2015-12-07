//! Interrupts support

use core::ptr;
use core::mem::size_of;
use arch::pic::ChainedPics;
use super::irq;
use super::descriptor_tables;
use spin::Mutex;

const IDT_ENTRY_COUNT: usize = 256;

#[allow(dead_code)]
extern {
    /// Offset of the code segment in the GDT defined in assembly.
    static gdt64_code_offset: u16;

    /// A dummy interrupt handler defined in assembly.
    fn dummy_interrupt_handler();

    /// The interrupts handlers table.
    static interrupt_handlers: [*const u8; IDT_ENTRY_COUNT];
}

/// The context on the stack available when the interrupt handler is called
#[repr(C, packed)]
pub struct InterruptStackContext {
    rsi: u64,
    rdi: u64,
    r11: u64,
    r10: u64,
    r9: u64,
    r8: u64,
    rdx: u64,
    rcx: u64,
    rax: u64,
    interrupt_id: u32,
    _interrupt_id_pad: u32,
    error_code: u32,
    _error_code_pad: u32,
}

/// The interface to the Programmable Controller Interface chip.
///
/// They handle hardware interrupts from 0x20 to master PIC1 and 
/// from 0x28 to chained PIC2.
static PICS: Mutex<ChainedPics> = Mutex::new(unsafe {
    ChainedPics::new(0x20, 0x28)
});

/// Print some useful information about CPU standard exceptions, if they happen.
fn cpu_interrupt_handler(context: &InterruptStackContext) {
    println!("{}, error 0x{:x}", 
        irq::CPU_EXCEPTIONS[context.interrupt_id as usize],
        context.error_code);

    // TODO: Print more useful information for specific interrupts, i.e. Page Faults.
    match context.interrupt_id {
        14 => {
            let err = irq::PageFaultException::from_bits(context.error_code);
            println!("{:?}", err);
        }
        _ => {}
    }

    loop {}
}

/// Eventually called from the assembly code to handle an interrupt.
#[no_mangle]
pub unsafe extern "C" fn rust_interrupt_handler(context: &InterruptStackContext) {
    // List of general IBM-PC Compatible Interrupt Information here: 
    // http://wiki.osdev.org/Interrupts
    match context.interrupt_id {
        0x00...0x1F => cpu_interrupt_handler(context),
        0x20 => { /* Timer */ }
        0x21 => {
            use arch::keyboard::STATE;
            if let Some(char) = STATE.lock().get_char() {
                if char == '\r' {
                    println!("");
                } else {
                    print!("{}", char);
                }
            }
        }
        0x22 => { /* Cascade to PIC2, never raised */ }
        0x23 => { /* COM2 */ }
        0x24 => { /* COM1 */ }
        0x25 => { /* LPT2 */ }
        0x26 => { /* Floppy Disk */ }
        0x27 => { /* LPT1 (Unreliable spurious interrupts) */ }
        0x28 => { /* CMOS real-time clock */ }
        0x29 => { /* Free for peripherals / Legacy SCSI / NIC */ }
        0x2A => { /* Free for peripherals / SCSI / NIC */ }
        0x2B => { /* Free for peripherals / SCSI / NIC */ }
        0x2C => { /* PS2 Mouse */ }
        0x2D => { /* FPU / Coprocessor / Inter-processor */ }
        0x2E => { /* Primary ATA Hard Disk */ }
        0x2F => { /* Secondary ATA Hard Disk */ }
        0x80 => {
            // Generally used for software interrupts on Unix-like OSes
            println!("Not Unix ;)");
        }
        _ => {
            println!("Unknown Interrupt #{}", context.interrupt_id);
            loop {}
        }
    }

    PICS.lock().end_of_interrupt(context.interrupt_id as u8);
}


/// Represent an Interrupt Descitptor Table (IDT) entry.
///
/// Sources: 
/// http://www.intel.com/Assets/en_US/PDF/manual/253668.pdf 6.14.1 for dets
/// https://littleosbook.github.io/
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct IdtEntry {
    offset_low: u16,
    segment_selector: u16,
    _reserved_0: u8,
    flags: u8,
    offset_high: u64,   // 48 bits used, last 16 LSBs are 0 (little endian)
    _reserved_1: u16,   // Must be 0
}

impl IdtEntry {
    /// Create an empty handler for static initialisation of an IDT table.
    pub const fn missing_handler() -> Self {
        IdtEntry {
            offset_low: 0,
            segment_selector: 0,
            _reserved_0: 0,
            flags: 0,
            offset_high: 0,
            _reserved_1: 0,
        }
    }

    /// Create a new entry given the gdt code selector offset and an handler.
    pub fn new(gdt_code_selector: u16, handler: *const u8) -> Self {
        IdtEntry {
            offset_low: ((handler as u64) & 0xFFFF) as u16,
            segment_selector: gdt_code_selector,
            _reserved_0: 0,
            // "Present" bit set.
            // Bit 0-4: D is 1 (handler in memory), 110 by default.
            flags: 0b1000_1110,
            offset_high: (handler as u64) >> 16,
            _reserved_1: 0
        }
    }
}

/// Enable interrupts.
pub unsafe fn enable() {
    asm!("sti");
}

/// Disable interrupts.
#[allow(dead_code)]
pub unsafe fn disable() {
    asm!("cli");
}

/// Generates a software interrupt.
#[macro_export]
macro_rules! int {
    ($x:expr) => {
        {
            asm!("int $0" :: "N"($x));
        }
    };
}

/// Represent the Interrupt Descriptor Table, which describes a list 
/// of interrupt handlers.
///
/// There are 255 interrupts and interrupt i is handled by the handler at ith 
/// position in the table.
/// After generating the table, loading the address of the handlers into the
/// right index in the table (handlers are defined in assembly and a 
/// pointer to them is exported so it can be read here), the table needs to
/// be loaded in memory so the CPU is aware of it. Use the `lidt` assembly 
/// instruction for that
struct Idt {
    table: [IdtEntry; IDT_ENTRY_COUNT],
}

impl Idt {
    /// Init the IDT table.
    pub unsafe fn init(&mut self) {
        self.add_handlers();
        self.load();
    }

    /// Fetch handlers addresses from memory (they're defined in assembly).
    fn add_handlers(&mut self) {
        for (index, &handler) in interrupt_handlers.iter().enumerate() {
            if handler != ptr::null() {
                self.table[index] = IdtEntry::new(gdt64_code_offset, handler);
            }
        }
    }

    /// Load the IDT table into memory.
    unsafe fn load(&self) {
        let idt_pointer = descriptor_tables::DescriptorTablePointer {
            base: &self.table[0] as *const IdtEntry as u64,
            limit: (size_of::<IdtEntry>() * IDT_ENTRY_COUNT) as u16,
        };
        descriptor_tables::lidt(&idt_pointer);
    }
}

static IDT: Mutex<Idt> = Mutex::new(
    Idt { table: [IdtEntry::missing_handler(); IDT_ENTRY_COUNT] }
);

/// Test a software input.
#[allow(dead_code)]
unsafe fn test_interrupt() {
    println!("Triggering interrupt.");
    int!(0x80);
    println!("Interrupt handled.");
}

/// Initialize interrupts.
pub unsafe fn init() {
    PICS.lock().init();
    
    IDT.lock().init();

    // Test software interrupts
    test_interrupt();

    // Enable real interrupts
    enable();
}
