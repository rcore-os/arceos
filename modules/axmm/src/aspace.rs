use core::fmt;

use axerrno::{ax_err, AxResult};
use axhal::paging::{MappingFlags, PageTable};
use memory_addr::{PhysAddr, VirtAddr};

use crate::paging_err_to_ax_err;

/// The virtual memory address space.
pub struct AddrSpace {
    base: VirtAddr,
    end: VirtAddr,
    pt: PageTable,
}

impl AddrSpace {
    /// Returns the address space base.
    pub const fn base(&self) -> VirtAddr {
        self.base
    }

    /// Returns the address space end.
    pub const fn end(&self) -> VirtAddr {
        self.end
    }

    /// Returns the address space size.
    pub const fn size(&self) -> usize {
        self.end.as_usize() - self.base.as_usize()
    }

    /// Returns the reference to the inner page table.
    pub const fn page_table(&self) -> &PageTable {
        &self.pt
    }

    /// Returns the root physical address of the inner page table.
    pub const fn page_table_root(&self) -> PhysAddr {
        self.pt.root_paddr()
    }

    /// Checks if the address space contains the given virtual address.
    pub const fn contains(&self, addr: VirtAddr) -> bool {
        self.base.as_usize() <= addr.as_usize() && addr.as_usize() < self.end.as_usize()
    }

    /// Checks if the address space contains the given virtual address range.
    pub const fn contains_range(&self, start: VirtAddr, size: usize) -> bool {
        self.base.as_usize() <= start.as_usize() && start.as_usize() + size < self.end.as_usize()
    }

    /// Checks if the address space overlaps with the given virtual address range.
    pub const fn overlaps_with(&self, start: VirtAddr, size: usize) -> bool {
        let end = start.as_usize() + size;
        !(end <= self.base.as_usize() || start.as_usize() >= self.end.as_usize())
    }

    /// Creates a new empty address space.
    pub(crate) fn new_empty(base: VirtAddr, size: usize) -> AxResult<Self> {
        Ok(Self {
            base,
            end: base + size,
            pt: PageTable::try_new().map_err(paging_err_to_ax_err)?,
        })
    }

    /// Copies page table mappings from another address space.
    ///
    /// It copies the page table entries only rather than the memory regions,
    /// usually usually used to copy a portion of the kernel space mapping to
    /// the user space.
    pub fn copy_mappings_from(&mut self, other: &AddrSpace) -> AxResult {
        if self.overlaps_with(other.base(), other.size()) {
            return ax_err!(InvalidInput, "address space overlap");
        }
        self.pt.copy_from(&other.pt, other.base(), other.size());
        Ok(())
    }

    /// Add a new fixed mapping for the specified virtual and physical address
    /// range.
    ///
    /// The mapping is linear, i.e., `start_vaddr` is mapped to `start_paddr`,
    /// and `start_vaddr + size` is mapped to `start_paddr + size`.
    ///
    /// The `flags` parameter specifies the mapping permissions and attributes.
    pub fn map_fixed(
        &mut self,
        start_vaddr: VirtAddr,
        start_paddr: PhysAddr,
        size: usize,
        flags: MappingFlags,
    ) -> AxResult {
        if !self.contains_range(start_vaddr, size) {
            return ax_err!(InvalidInput, "address out of range");
        }
        self.pt
            .map_region(start_vaddr, start_paddr, size, flags, true)
            .map_err(paging_err_to_ax_err)?;
        Ok(())
    }

    /// Removes the mappings for the specified virtual address range.
    pub fn unmap(&mut self, start: VirtAddr, size: usize) -> AxResult {
        if !self.contains_range(start, size) {
            return ax_err!(InvalidInput, "address out of range");
        }
        self.pt
            .unmap_region(start, size)
            .map_err(paging_err_to_ax_err)?;
        Ok(())
    }
}

impl fmt::Debug for AddrSpace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AddrSpace")
            .field("va_range", &(self.base.as_usize()..self.end.as_usize()))
            .field("page_table_root", &self.pt.root_paddr())
            .finish()
    }
}
