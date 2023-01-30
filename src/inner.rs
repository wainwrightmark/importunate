use core::fmt::Debug;
use core::hash::Hash;
use core::ops::Range;

use num::{Integer, Unsigned};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use static_assertions::const_assert;

/// The inner type of a permutation
pub trait Inner:
    Copy
    + Clone
    + Debug
    + PartialEq
    + Eq
    + Hash
    + Default
    + Unsigned
    + Integer
    + TryFrom<usize>
    + TryInto<usize>
    + core::iter::Product
{
    const MAX_ELEMENTS: usize;
    type Iterator: Iterator<Item = Self>;

    fn get_permutation_range(elements: usize) -> Self::Iterator;

    fn get_factorial(n: usize) -> Self;
}

macro_rules! impl_permutation_inner {
    ($inner:ty, $max_elements:tt, $arr_len: tt ) => {
        impl Inner for $inner {
            const MAX_ELEMENTS: usize = $max_elements;

            type Iterator = Range<Self>;

            fn get_permutation_range(elements: usize) -> Self::Iterator {
                0..(Self::get_factorial(elements))
            }

            fn get_factorial(n: usize) -> Self {
                const fn make_arr() -> [$inner; $arr_len] {
                    let mut arr = [0; $arr_len];
                    let mut i = 0;
                    while i < $arr_len {
                        arr[i] = factorial(i as $inner);
                        i = i + 1;
                    }
                    arr
                }

                const fn factorial(i: $inner) -> $inner {
                    match i {
                        0 => 1,
                        1 => 1,
                        _ => i * (factorial(i - 1)),
                    }
                }

                const FACTORIALS: [$inner; $arr_len] = make_arr();
                FACTORIALS[n]
            }
        }
    };
}

impl_permutation_inner!(u8, 5, 6);
impl_permutation_inner!(u16, 8, 9);
impl_permutation_inner!(u32, 12, 13);
impl_permutation_inner!(u64, 20, 21);
impl_permutation_inner!(u128, 34, 35);
