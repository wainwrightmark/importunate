use core::fmt::Debug;
use core::hash::Hash;
use core::ops::Range;

use num_integer::Integer;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The inner type of a permutation
pub trait Inner:
    Copy
    + Clone
    + Debug
    + PartialEq
    + Eq
    + Hash
    + Default
    + Integer
    + TryFrom<usize>
    + TryInto<usize>
    + core::iter::Product
{
    const MAX_ELEMENTS: usize;
    const BYTES: usize;
    type Iterator: Iterator<Item = Self>;

    fn get_permutation_range(elements: usize) -> Self::Iterator;

    fn get_factorial(n: usize) -> Self;

    fn to_le_byte_array<const BYTES: usize>(&self) -> [u8; BYTES];

    fn from_le_byte_array(bytes: &[u8]) -> Self;
}

macro_rules! impl_permutation_inner {
    ($inner:ty, $max_elements:tt, $arr_len: tt, $bytes: tt ) => {
        impl Inner for $inner {
            const MAX_ELEMENTS: usize = $max_elements;
            const BYTES: usize = $bytes;

            type Iterator = Range<Self>;

            fn get_permutation_range(elements: usize) -> Self::Iterator {
                0..(Self::get_factorial(elements))
            }

            fn to_le_byte_array<const BYTES: usize>(&self) -> [u8; BYTES] {
                let bytes = self.to_le_bytes();
                let mut arr = [0u8; BYTES];
                for i in 0..(BYTES.min(arr.len())) {
                    arr[i] = bytes[i]
                }
                arr
            }

            fn from_le_byte_array(bytes: &[u8]) -> Self {
                let mut new_bytes = [0; Self::BYTES];

                for i in 0..(bytes.len().min(Self::BYTES)) {
                    new_bytes[i] = bytes[i]
                }

                Self::from_le_bytes(new_bytes)
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

impl_permutation_inner!(u8, 5, 6, 1);
impl_permutation_inner!(u16, 8, 9, 2);
impl_permutation_inner!(u32, 12, 13, 4);
impl_permutation_inner!(u64, 20, 21, 8);
impl_permutation_inner!(u128, 34, 35, 16);
