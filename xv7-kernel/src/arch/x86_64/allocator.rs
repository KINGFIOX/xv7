use crate::{
    config::*,
    memory::{FrameAllocator, FRAME_ALLOCATOR},
};
use x86_64::structures::paging::{Mapper, Page};
use x86_64::structures::paging::{PageTable, PageTableFlags, RecursivePageTable};
use x86_64::VirtAddr;

pub fn init_heap() {
    let mut frame_allocator = FRAME_ALLOCATOR.lock();
    let page_table = unsafe { &mut *(0xFFFF_FFFF_FFFF_F000 as *mut PageTable) };
    let mut mapper = RecursivePageTable::new(page_table).unwrap();

    let page_range = {
        let heap_start = VirtAddr::new(KERNEL_HEAP_BASE as u64);
        let heap_end = heap_start + KERNEL_HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator.allocate_frame().unwrap();
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_EXECUTE;
        unsafe {
            mapper
                .map_to(page, frame, flags, &mut *frame_allocator)
                .unwrap()
                .flush();
        }
    }

    unsafe {
        crate::allocator::ALLOCATOR
            .lock()
            .init(KERNEL_HEAP_BASE as usize, KERNEL_HEAP_SIZE);
    }
}
