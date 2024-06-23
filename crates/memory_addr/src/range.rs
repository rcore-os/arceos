use core::{fmt, ops::Range};

/// A generic range of addresses.
///
/// The range is inclusive on the start and exclusive on the end.
/// It is empty if `start >= end`.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct AddrRange<A>
where
    A: From<usize> + Into<usize> + Copy,
{
    /// The lower bound of the range (inclusive).
    pub start: A,
    /// The upper bound of the range (exclusive).
    pub end: A,
}

macro_rules! usize {
    ($addr:expr) => {
        Into::<usize>::into($addr)
    };
}

impl<A> AddrRange<A>
where
    A: From<usize> + Into<usize> + Copy,
{
    /// Creates a new address range.
    #[inline]
    pub const fn new(start: A, end: A) -> Self {
        Self { start, end }
    }

    /// Creates a new address range from the start address and the size.
    #[inline]
    pub const fn from_start_size(start: A, size: usize) -> Self {
        Self {
            start,
            end: A::from(usize!(start) + size),
        }
    }

    /// Returns `true` if the range is empty (`start >= end`).
    #[inline]
    pub const fn is_empty(self) -> bool {
        usize!(self.start) >= usize!(self.end)
    }

    /// Returns the size of the range.
    #[inline]
    pub const fn size(self) -> usize {
        self.end.into() - self.start.into()
    }

    /// Checks if the range contains the given address.
    #[inline]
    pub const fn contains(self, addr: A) -> bool {
        usize!(self.start) <= usize!(addr) && usize!(addr) < usize!(self.end)
    }

    /// Checks if the range contains the given address range.
    #[inline]
    pub const fn contains_range(self, other: Self) -> bool {
        usize!(self.start) <= usize!(other.start) && usize!(other.end) <= usize!(self.end)
    }

    /// Checks if the range is contained in the given address range.
    #[inline]
    pub const fn contained_in(self, other: Self) -> bool {
        other.contains_range(self)
    }

    /// Checks if the range overlaps with the given address range.
    #[inline]
    pub const fn overlaps(self, other: Self) -> bool {
        usize!(self.start) < usize!(other.end) && usize!(other.start) < usize!(self.end)
    }
}

impl<A, B> From<Range<B>> for AddrRange<A>
where
    A: From<usize> + Into<usize> + Copy,
    B: Into<A>,
{
    fn from(range: Range<B>) -> Self {
        Self::new(range.start.into(), range.end.into())
    }
}

impl<A> fmt::Debug for AddrRange<A>
where
    A: From<usize> + Into<usize> + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x?}..{:#x?}", usize!(self.start), usize!(self.end))
    }
}
