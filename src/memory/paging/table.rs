use core::ops::{Index, IndexMut};
use core::marker::PhantomData;

use memory::FrameAllocator;
use memory::paging::entry::*;
use memory::paging::ENTRY_COUNT;

pub const P4: *mut Table<Level4> = 0xffffffff_fffff000 as *mut _;

/// A paging table.
///
/// Each table has 2^9 = 512 entries.
///
/// The x86 architecture uses a 4-level page table in 64-bit mode.
/// The bits 48–63 are so-called sign extension bits and must be copies of bit 47.
/// The following 36 bits define the page table indexes (9 bits per table) and 
/// the last 12 bits specify the offset in the 4KiB page.
///
/// The P4 (PML4) entry points to a P3 (PDP) table, where the next 9 bits of the address 
/// are used to select an entry. The P3 entry then points to a P2 (PD) table and 
/// the P2 entry points to a P1 (PT) table. The P1 entry, which is specified 
/// through bits 12–20, finally points to the physical frame.
pub struct Table<L: TableLevel> {
    entries: [Entry; ENTRY_COUNT],
    level: PhantomData<L>,
}

impl<L> Table<L> where L: TableLevel
{
    pub fn zero(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.set_unused();
        }
    }
}

impl<L> Table<L> where L: HierachicalLevel
{
    fn next_table_address(&self, index: usize) -> Option<usize> {
        let entry_flags = self[index].flags();
        if entry_flags.contains(PRESENT) && !entry_flags.contains(HUGE_PAGE) {
            let table_address = self as *const _ as usize;
            Some((table_address << 9) | (index << 12))
        } else {
            None
        }
    }

    pub fn next_table(&self, index: usize) -> Option<&Table<L::NextLevel>> {
        self.next_table_address(index)
            .map(|address| unsafe { &*(address as *const _) })
    }

    pub fn next_table_mut(&mut self, index: usize) -> Option<&mut Table<L::NextLevel>> {
        self.next_table_address(index)
            .map(|address| unsafe { &mut *(address as *mut _) })
    }

    pub fn next_table_create<A>(&mut self,
                                index: usize,
                                allocator: &mut A)
                                -> &mut Table<L::NextLevel>
        where A: FrameAllocator
    {
        if self.next_table(index).is_none() {
            assert!(!self.entries[index].flags().contains(HUGE_PAGE),
                    "mapping code does not support huge pages");
            let frame = allocator.allocate_frame().expect("no frames available");
            self.entries[index].set(frame, PRESENT | WRITABLE);
            self.next_table_mut(index).unwrap().zero();
        }
        self.next_table_mut(index).unwrap()
    }
}

impl<L> Index<usize> for Table<L> where L: TableLevel
{
    type Output = Entry;

    fn index(&self, index: usize) -> &Entry {
        &self.entries[index]
    }
}

impl<L> IndexMut<usize> for Table<L> where L: TableLevel
{
    fn index_mut(&mut self, index: usize) -> &mut Entry {
        &mut self.entries[index]
    }
}

pub trait TableLevel {}

pub enum Level4 {}
enum Level3 {}
enum Level2 {}
enum Level1 {}

impl TableLevel for Level4 {}
impl TableLevel for Level3 {}
impl TableLevel for Level2 {}
impl TableLevel for Level1 {}

trait HierachicalLevel: TableLevel {
    type NextLevel: TableLevel;
}

impl HierachicalLevel for Level4 {
    type NextLevel = Level3;
}

impl HierachicalLevel for Level3 {
    type NextLevel = Level2;
}

impl HierachicalLevel for Level2 {
    type NextLevel = Level1;
}
