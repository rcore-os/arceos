use axhal::paging::{MappingFlags, PageTable};
use memory_addr::{PhysAddr, VirtAddr};
use memory_set::MappingBackend;

#[derive(Clone)]
pub struct FixedBackend {
    pa_va_offset: usize,
}

impl MappingBackend<MappingFlags, PageTable> for FixedBackend {
    fn map(&self, start: VirtAddr, size: usize, flags: MappingFlags, pt: &mut PageTable) -> bool {
        let pa_start = PhysAddr::from(start.as_usize() - self.pa_va_offset);
        pt.map_region(start, pa_start, size, flags, false).is_ok()
    }

    fn unmap(&self, start: VirtAddr, size: usize, pt: &mut PageTable) -> bool {
        pt.unmap_region(start, size).is_ok()
    }
}

impl FixedBackend {
    pub const fn new(pa_va_offset: usize) -> Self {
        Self { pa_va_offset }
    }
}
