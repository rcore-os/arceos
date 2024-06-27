use crate::VirtAddr;

/// A page-by-page iterator.
///
/// The page size is specified by the generic parameter `PAGE_SIZE`, which must
/// be a power of 2.
pub struct PageIter<const PAGE_SIZE: usize> {
    start: VirtAddr,
    end: VirtAddr,
}

impl<const PAGE_SIZE: usize> PageIter<PAGE_SIZE> {
    /// Creates a new [`PageIter`].
    ///
    /// Returns `None` if `PAGE_SIZE` is not a power of 2, or `start` or `end`
    /// is not page-aligned.
    pub const fn new(start: VirtAddr, end: VirtAddr) -> Option<Self> {
        if !PAGE_SIZE.is_power_of_two()
            || !start.is_aligned(PAGE_SIZE)
            || !end.is_aligned(PAGE_SIZE)
        {
            None
        } else {
            Some(Self { start, end })
        }
    }
}

impl<const PAGE_SIZE: usize> Iterator for PageIter<PAGE_SIZE> {
    type Item = VirtAddr;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let ret = self.start;
            self.start += PAGE_SIZE;
            Some(ret)
        } else {
            None
        }
    }
}
