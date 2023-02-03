use core::fmt::Debug;
use core::hash::Hash;
use core::ops::Range;

use num_integer::Integer;

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
    + From<u8>
    + TryFrom<usize>
    + TryInto<u8>
    + TryInto<usize>
    + core::iter::Product
{
    /// The maximum number of elements a permutation of this size can have
    const MAX_ELEMENTS: usize;
    /// The size of this type in bytes
    const BYTES: usize;

    /// Range iterator for this type
    type RangeIter: Iterator<Item = Self>;

    #[must_use]
    /// Get the range of legal permutations for a given number of elements
    fn get_permutation_range(elements: usize) -> Self::RangeIter;

    #[must_use]
    /// Get the factorial of a given number
    fn get_factorial(n: usize) -> Self;

    #[must_use]
    /// Convert to a byte array of a given length, truncating if necessary
    fn to_le_byte_array<const BYTES: usize>(&self) -> [u8; BYTES];

    #[must_use]
    /// Create from a little endian byte array
    fn from_le_byte_array(bytes: &[u8]) -> Self;
}

macro_rules! impl_permutation_inner {
    ($inner:ty, $max_elements:tt, $arr_len: tt, $bytes: tt ) => {
        impl Inner for $inner {
            const MAX_ELEMENTS: usize = $max_elements;
            const BYTES: usize = $bytes;

            type RangeIter = Range<Self>;

            fn get_permutation_range(elements: usize) -> Self::RangeIter {
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
                        i += 1;
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
