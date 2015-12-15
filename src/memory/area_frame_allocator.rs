use memory::{Frame, FrameAllocator};
use multiboot2::{MemoryAreaIter, MemoryArea};

/// A frame allocator that uses the memory areas from the multiboot information structure as
/// source. The {kernel, multiboot}_{start, end} fields are used to avoid returning memory that is
/// already in use.
///
/// `kernel_end` and `multiboot_end` are _inclusive_ bounds.
pub struct AreaFrameAllocator {
    next_free_frame: Frame,
    current_area: Option<&'static MemoryArea>,
    areas: MemoryAreaIter,
    kernel_start: Frame,
    kernel_end: Frame,
    multiboot_start: Frame,
    multiboot_end: Frame,
}

impl AreaFrameAllocator {
    pub fn new(kernel_start: usize, kernel_end: usize,
        multiboot_start: usize, multiboot_end: usize,
        memory_areas: MemoryAreaIter) -> Self {
        let mut allocator = AreaFrameAllocator {
            next_free_frame: Frame::containing_address(0),
            current_area: None,
            areas: memory_areas,
            kernel_start: Frame::containing_address(kernel_start),
            kernel_end: Frame::containing_address(kernel_end),
            multiboot_start: Frame::containing_address(multiboot_start),
            multiboot_end: Frame::containing_address(multiboot_end),
        };
        allocator.choose_next_area();
        allocator
    }

    fn choose_next_area(&mut self) {
        self.current_area = self.areas.clone().filter(|area| {
            let address = area.base_addr + area.length - 1;
            Frame::containing_address(address as usize) >= self.next_free_frame
        }).min_by(|area| area.base_addr);

        if let Some(area) = self.current_area {
            let start_frame = Frame::containing_address(area.base_addr as usize);
            if self.next_free_frame < start_frame {
                self.next_free_frame = start_frame;
            }
        }
    }
}

impl FrameAllocator for AreaFrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame> {
        if let Some(area) = self.current_area {
            let frame = Frame { number: self.next_free_frame.number };

            let current_area_last_frame = {
                let address = area.base_addr + area.length - 1;
                Frame::containing_address(address as usize)
            };

            if frame > current_area_last_frame {
                // All frames of current area are used, get next area
                self.choose_next_area();
            } else if frame >= self.kernel_start && frame <= self.kernel_end {
                // Selected `frame` is used by the kernel, move at the end of kernel memory
                self.next_free_frame = Frame {
                    number: self.kernel_end.number + 1
                };
            } else if frame >= self.multiboot_start && frame <= self.multiboot_end {
                // Selected `frame` is used by the multiboot, move at the end of multiboot memory
                self.next_free_frame = Frame {
                    number: self.multiboot_end.number + 1
                };
            } else {
                // Frame is unused, increment `next_free_frame` and return it
                self.next_free_frame.number += 1;
                return Some(frame);
            }

            // If we get here, `frame` was not valid, try again with new `next_free_frame`
            self.allocate_frame()
        } else {
            // If we get here, there are no more frames left
            None
        }
    }

    fn deallocate_frame(&mut self, frame: Frame) {
        unimplemented!()
    }
}