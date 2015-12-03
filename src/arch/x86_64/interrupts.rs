
use arch::pic::ChainedPics;
use super::irq;
use spin::Mutex;

const IDT_ENTRY_COUNT:usize = 256;

#[allow(dead_code)]
extern {
    static gdt64_code_offset: u16;

    fn dummy_interrupt_handler();

    static interrupt_handlers: [*const u8; IDT_ENTRY_COUNT];
}

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

    // TODO: Print more useful information for specific interrupts, i.e. Page Faults

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
        0x21 => { /* Keyboard */ }
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


/// TODO: Represent an Interrupt Descitptor Table (IDT) entry
/// An entry in the IDT for an interrupt handler consists of 64 bits. The highest 32 bits:
/// Bit:     | 31      16 | 15 | 14 13 | 12 | 11 | 10 9 8 | 7 6 5 | 4 3 2 1 0 |
/// Content: | hoffset    | P  | DPL   | 0  | D  | 1  1 0 | 0 0 0 | reserved  |
/// The lowest 32 bits:
/// Bit:     | 31              16 | 15              0 |
/// Content: | segment selector   | offset low        |
/// This assumes the usage of segmentation. With paging is slightly different.
/// See http://www.intel.com/Assets/en_US/PDF/manual/253668.pdf 6.14.1 for dets
/// Source: https://littleosbook.github.io/
struct IdtEntry;
//{
    // add code here
//}

/// TODO: Represent the Interrupt Descriptor Table, which describes a list 
/// of interrupt handlers.
///
/// There are 255 interrupts and interrupt i is handled by the handler at ith 
/// position in the table.
/// After generating the table, loading the address of the handlers into the
/// right index in the table (handlers are defined in assembly and a 
/// pointer to them is exported so it can be read here), the table needs to
/// be loaded in memory so the CPU is aware of it. Use the `lidt` assembly 
/// instruction for that
struct Idt;
//{
    // add code here
//}

//static IDT: Mutex<Idt> = Mutex::new(
//    Idt { table: [missing_handler(); IDT_ENTRY_COUNT] }
//);

/// TODO: Initialize interrupts
/// 1) Set up PIC
/// 2) Initialize IDT
/// 3) Load IDT
/// 4) Enable Interrupts
pub unsafe fn init() {
    PICS.lock().init();
    // add code here
}

// TODO: Represents a interrupt dummy handler for static contruction of the IDT
const fn missing_handler() -> IdtEntry {
    // add code here
    IdtEntry
}