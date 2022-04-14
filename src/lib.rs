mod non_zero_u63;

use non_zero_u63::NonZeroU63;
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

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_log2_ceil_v1_limits() {
        for x in [0, 1, 2, 3, 4, 5, u64::MAX - 1, u64::MAX] {
            assert_eq!(log2_ceil_baseline(x), log2_ceil_v1(x));
        }
    }

    #[test]
    fn test_log2_ceil_v2_limits() {
        for x in [1, 2, 3, 4, 5, u64::MAX - 1, u64::MAX] {
            let x = NonZeroU64::new(x).unwrap();
            assert_eq!(log2_ceil_baseline(x.get()), log2_ceil_v2(x));
        }
    }

    #[test]
    fn test_log2_ceil_v3_limits() {
        for x in [1, 2, 3, 4, 5, NonZeroU63::MAX - 1, NonZeroU63::MAX] {
            let x = NonZeroU63::new(x).unwrap();
            assert_eq!(log2_ceil_baseline(x.get()), log2_ceil_v3(x));
        }
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
