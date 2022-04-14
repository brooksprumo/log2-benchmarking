#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct NonZeroU63(u64);

impl NonZeroU63 {
    pub const BITS: u32 = u64::BITS - 1;
    pub const MIN: u64 = 1;
    pub const MAX: u64 = (1 << Self::BITS) - 1;

    #[must_use]
    #[inline]
    pub const unsafe fn new_unchecked(n: u64) -> Self {
        // SAFETY: this is guaranteed to be safe by the caller.
        Self(n)
    }

    #[must_use]
    #[inline]
    pub const fn new(n: u64) -> Option<Self> {
        if n >= Self::MIN && n <= Self::MAX {
            // SAFETY: we just checked that the value is in range
            Some(Self(n))
        } else {
            None
        }
    }

    #[inline]
    pub const fn get(self) -> u64 {
        self.0
    }
}
