//! Physical memory management.

#[doc(no_inline)]
pub use os_memory::{BootState, BusAddr, MemRegion, MemRegionFlags, PhysAddr, VirtAddr};

#[doc(no_inline)]
pub use memory_addr::PAGE_SIZE_4K;

/// Converts a virtual address to a physical address.
///
/// It assumes that there is a linear mapping with the offset
/// [`PHYS_VIRT_OFFSET`], that maps all the physical memory to the virtual
/// space at the address plus the offset. So we have
/// `paddr = vaddr - PHYS_VIRT_OFFSET`.
///
/// [`PHYS_VIRT_OFFSET`]: axconfig::PHYS_VIRT_OFFSET
#[inline]
pub const fn virt_to_phys(vaddr: VirtAddr) -> PhysAddr {
    PhysAddr::from(vaddr.as_usize() - axconfig::PHYS_VIRT_OFFSET)
}

/// Converts a physical address to a virtual address.
///
/// It assumes that there is a linear mapping with the offset
/// [`PHYS_VIRT_OFFSET`], that maps all the physical memory to the virtual
/// space at the address plus the offset. So we have
/// `vaddr = paddr + PHYS_VIRT_OFFSET`.
///
/// [`PHYS_VIRT_OFFSET`]: axconfig::PHYS_VIRT_OFFSET
#[inline]
pub const fn phys_to_virt(paddr: PhysAddr) -> VirtAddr {
    VirtAddr::from(paddr.as_usize() + axconfig::PHYS_VIRT_OFFSET)
}

/// Returns an iterator over all physical memory regions.
pub fn memory_regions() -> impl Iterator<Item = MemRegion> {
    kernel_image_regions().chain(crate::platform::mem::platform_regions())
}

/// Returns the memory regions of the kernel image (code and data sections).
fn kernel_image_regions() -> impl Iterator<Item = MemRegion> {
    [
        MemRegion {
            paddr: virt_to_phys((_stext as usize).into()),
            size: _etext as usize - _stext as usize,
            flags: MemRegionFlags::RESERVED | MemRegionFlags::READ | MemRegionFlags::EXECUTE,
            name: ".text",
        },
        MemRegion {
            paddr: virt_to_phys((_srodata as usize).into()),
            size: _erodata as usize - _srodata as usize,
            flags: MemRegionFlags::RESERVED | MemRegionFlags::READ,
            name: ".rodata",
        },
        MemRegion {
            paddr: virt_to_phys((_sdata as usize).into()),
            size: _edata as usize - _sdata as usize,
            flags: MemRegionFlags::RESERVED | MemRegionFlags::READ | MemRegionFlags::WRITE,
            name: ".data .tdata .tbss .percpu",
        },
        MemRegion {
            paddr: virt_to_phys((boot_stack as usize).into()),
            size: boot_stack_top as usize - boot_stack as usize,
            flags: MemRegionFlags::RESERVED | MemRegionFlags::READ | MemRegionFlags::WRITE,
            name: "boot stack",
        },
        MemRegion {
            paddr: virt_to_phys((_sbss as usize).into()),
            size: _ebss as usize - _sbss as usize,
            flags: MemRegionFlags::RESERVED | MemRegionFlags::READ | MemRegionFlags::WRITE,
            name: ".bss",
        },
    ]
    .into_iter()
}

/// Returns the default MMIO memory regions (from [`axconfig::MMIO_REGIONS`]).
#[allow(dead_code)]
pub(crate) fn default_mmio_regions() -> impl Iterator<Item = MemRegion> {
    axconfig::MMIO_REGIONS.iter().map(|reg| MemRegion {
        paddr: reg.0.into(),
        size: reg.1,
        flags: MemRegionFlags::RESERVED
            | MemRegionFlags::DEVICE
            | MemRegionFlags::READ
            | MemRegionFlags::WRITE,
        name: "mmio",
    })
}

/// Returns the default free memory regions (kernel image end to physical memory end).
#[allow(dead_code)]
pub(crate) fn default_free_regions() -> impl Iterator<Item = MemRegion> {
    let start = virt_to_phys((_ekernel as usize).into()).align_up_4k();
    let end = PhysAddr::from(axconfig::PHYS_MEMORY_END).align_down_4k();
    core::iter::once(MemRegion {
        paddr: start,
        size: end.as_usize() - start.as_usize(),
        flags: MemRegionFlags::FREE | MemRegionFlags::READ | MemRegionFlags::WRITE,
        name: "free memory",
    })
}

/// Fills the `.bss` section with zeros.
#[allow(dead_code)]
pub(crate) fn clear_bss() {
    unsafe {
        core::slice::from_raw_parts_mut(_sbss as usize as *mut u8, _ebss as usize - _sbss as usize)
            .fill(0);
    }
}

extern "C" {
    fn _stext();
    fn _etext();
    fn _srodata();
    fn _erodata();
    fn _sdata();
    fn _edata();
    fn _sbss();
    fn _ebss();
    fn _ekernel();
    fn boot_stack();
    fn boot_stack_top();
}
