use std::num::NonZeroU64;

pub fn log2_ceil_baseline(x: u64) -> u32 {
    (x as f64).log2().ceil() as u32
}

pub fn log2_ceil_v1(x: u64) -> u32 {
    if x == 0 {
        0
    } else if x < NonZeroU63::MAX {
        let x = unsafe { NonZeroU63::new_unchecked(x) };
        log2_ceil_v3(x)
    } else {
        let x = unsafe { NonZeroU64::new_unchecked(x) };
        log2_ceil_v2(x)
    }
}

pub fn log2_ceil_v2(x: NonZeroU64) -> u32 {
    u64::BITS - (x.get() - 1).leading_zeros()
}

pub fn log2_ceil_v3(x: NonZeroU63) -> u32 {
    NonZeroU63::BITS - (2 * x.get() - 1).leading_zeros()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct NonZeroU63(u64);

impl NonZeroU63 {
    const BITS: u32 = u64::BITS - 1;
    const MIN: u64 = 1;
    const MAX: u64 = (1 << Self::BITS) - 1;

    #[must_use]
    #[inline]
    pub const unsafe fn new_unchecked(n: u64) -> Self {
        // SAFETY: this is guaranteed to be safe by the caller.
        unsafe { Self(n) }
    }

    #[must_use]
    #[inline]
    pub const fn new(n: u64) -> Option<Self> {
        if n >= Self::MIN && n <= Self::MAX {
            // SAFETY: we just checked that the value is in range
            Some(unsafe { Self(n) })
        } else {
            None
        }
    }

    #[inline]
    pub const fn get(self) -> u64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_log2_ceil_v1_limits() {
        let min = u64::MIN;
        assert_eq!(log2_ceil_baseline(min), log2_ceil_v1(min));
        let max = u64::MAX;
        assert_eq!(log2_ceil_baseline(max), log2_ceil_v1(max));
    }

    #[test]
    fn test_log2_ceil_v2_limits() {
        let min = NonZeroU64::new(1).unwrap();
        assert_eq!(log2_ceil_baseline(min.get()), log2_ceil_v2(min));
        let max = NonZeroU64::new(u64::MAX).unwrap();
        assert_eq!(log2_ceil_baseline(max.get()), log2_ceil_v2(max));
    }

    #[test]
    fn test_log2_ceil_v3_limits() {
        let min = NonZeroU63::new(NonZeroU63::MIN).unwrap();
        assert_eq!(log2_ceil_baseline(min.get()), log2_ceil_v3(min));
        let max = NonZeroU63::new(NonZeroU63::MAX).unwrap();
        assert_eq!(log2_ceil_baseline(max.get()), log2_ceil_v3(max));
    }

    proptest! {
        #[test]
        fn pbt_log2_ceil_v1(x in any::<u64>()) {
            prop_assert_eq!(
                log2_ceil_baseline(x),
                log2_ceil_v1(x)
            );
        }

        #[test]
        fn pbt_log2_ceil_v2(x in any::<u64>()) {
            if let Some(x) = NonZeroU64::new(x) {
                prop_assert_eq!(
                    log2_ceil_baseline(x.get()),
                    log2_ceil_v2(x)
                );
            }
        }

        #[test]
        fn pbt_log2_ceil_v3(x in any::<u64>()) {
            if let Some(x) = NonZeroU63::new(x) {
                prop_assert_eq!(
                    log2_ceil_baseline(x.get()),
                    log2_ceil_v3(x)
                );
            }
        }
    }
}
