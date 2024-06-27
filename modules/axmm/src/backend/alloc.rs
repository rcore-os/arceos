use axalloc::global_allocator;
use axhal::mem::{phys_to_virt, virt_to_phys};
use axhal::paging::{MappingFlags, PageSize, PageTable};
use memory_addr::{PageIter4K, VirtAddr, PAGE_SIZE_4K};

use super::Backend;

impl Backend {
    /// Creates a new allocation mapping backend.
    pub const fn new_alloc(populate: bool) -> Self {
        Self::Alloc { populate }
    }

    pub(crate) fn map_alloc(
        &self,
        start: VirtAddr,
        size: usize,
        flags: MappingFlags,
        pt: &mut PageTable,
        _populate: bool,
    ) -> bool {
        debug!("map_alloc: [{:#x}, {:#x}) {:?}", start, start + size, flags);
        for addr in PageIter4K::new(start, start + size).unwrap() {
            if let Ok(vaddr) = global_allocator().alloc_pages(1, PAGE_SIZE_4K) {
                let frame = virt_to_phys(vaddr.into());
                if pt.map(addr, frame, PageSize::Size4K, flags).is_err() {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    pub(crate) fn unmap_alloc(
        &self,
        start: VirtAddr,
        size: usize,
        pt: &mut PageTable,
        _populate: bool,
    ) -> bool {
        debug!("unmap_alloc: [{:#x}, {:#x})", start, start + size);
        for addr in PageIter4K::new(start, start + size).unwrap() {
            if let Ok((frame, page_size)) = pt.unmap(addr) {
                // Deallocate the physical frame if there is a mapping in the
                // page table.
                if page_size.is_huge() {
                    return false;
                }
                let vaddr = phys_to_virt(frame);
                global_allocator().dealloc_pages(vaddr.as_usize(), 1);
            } else {
                // It's fine if the page is not mapped.
            }
        }
        true
    }
}
