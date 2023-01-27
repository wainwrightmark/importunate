#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![doc(html_root_url = "https://docs.rs/importunate/0.4.0")]
#![allow(missing_docs)]
#![allow(warnings, dead_code, unused_imports, unused_mut)]
#![warn(clippy::pedantic)]

//! [![github]](https://github.com/wainwrightmark/importunate)&ensp;[![crates-io]](https://crates.io/crates/importunate)&ensp;[![docs-rs]](https://docs.rs/importunate)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! Methods for choosing random elements from an iterator.
//!
//! <br>
//!
//! ## Usage
//!
//! ```
//! use importunate::*;
//!
//!
//! ```
//!
//! ## Readme Docs
//!
//! You can find the crate's readme documentation on the
//! [crates.io] page, or alternatively in the [`README.md`] file on the GitHub project repo.
//!
//! [crates.io]: https://crates.io/crates/importunate
//! [`README.md`]: https://github.com/wainwrightmark/importunate

// TODO
// multiplication
// inverse
// element at index
// index of element
// create from array
// documentation
// benchmarking

use core::fmt::Debug;
use core::hash::Hash;
use core::ops::Range;

use num::{Integer, Unsigned};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use static_assertions::const_assert;

/// A permutation
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Permutation<Inner: PermutationInner, const ELEMENTS: usize>(Inner);

/// The inner type of a permutation
pub trait PermutationInner:
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
    fn divmod(self, length: usize) -> (Self, usize) {
        let l: Self = length.try_into().ok().unwrap();
        let (s, r) = self.div_rem(&l);

        let r: usize = r.try_into().ok().unwrap();
        (s, r)
    }

    const MAX_ELEMENTS: usize;

    fn get_permutation_max(elements: usize) -> Self {
        debug_assert!(elements <= elements);
        if elements == 0 {
            return Self::one();
        }
        let num: Self = elements.try_into().ok().unwrap();

        let mut max = Self::one();
        let mut s = Self::one();
        while s <= num {
            max = max * s;
            s = s + Self::one();
        }
        max
    }

    fn get_permutation_range(elements: usize) -> Range<Self> {
        Self::zero()..(Self::get_permutation_max(elements))
    }

    fn get_factorial(n: usize) -> Self;
}

macro_rules! impl_permutation_inner {
    ($inner:ty, $max_elements:tt, $arr_len: tt ) => {
        impl PermutationInner for $inner {
            const MAX_ELEMENTS: usize = $max_elements;

            fn get_factorial(n: usize) -> Self {
                use array_const_fn_init::array_const_fn_init;

                const fn factorial1(i: usize) -> $inner {
                    factorial(i as $inner)
                }

                const fn factorial(i: $inner) -> $inner {
                    match i {
                        0 => 1,
                        1 => 1,
                        _ => i * (factorial(i - 1)),
                    }
                }

                const FACTORIALS: [$inner; $arr_len] =
                    array_const_fn_init![factorial1; $arr_len];
                FACTORIALS[n]
            }
        }
    };
}

impl_permutation_inner!(u8, 5,6);
impl_permutation_inner!(u16, 8,9);
impl_permutation_inner!(u32, 12,13);
impl_permutation_inner!(u64, 20,21);
impl_permutation_inner!(u128, 34,35);

impl<Inner: PermutationInner, const Elements: usize> From<Inner> for Permutation<Inner, Elements> {
    fn from(value: Inner) -> Self {
        debug_assert!(Elements <= Inner::MAX_ELEMENTS);
        Self(value)
    }
}

impl<Inner: PermutationInner, const Elements: usize> Default for Permutation<Inner, Elements> {
    fn default() -> Self {
        debug_assert!(Elements <= Inner::MAX_ELEMENTS);
        Self(Default::default())
    }
}

fn get_index_of(arr: &[usize], e: usize) -> usize {
    for i in 0..arr.len() {
        if arr[i] == e {
            return i;
        }
    }
    panic!("Could not find index {e}")
}

impl<Inner: PermutationInner, const Elements: usize> Permutation<Inner, Elements> {
    /// Apply this permutation to an array, reordering the first `Elements` elements
    pub fn apply<T>(&self, arr: &mut [T]) {
        let mut rem = self.0;
        let len = arr.len().min(Elements);

        for i in (0..len) {
            let (r, diff) = rem.divmod(len - i);
            rem = r;
            arr.swap(i, diff + i);
        }
    }

    pub fn get_inverse_factorial(n: usize) -> Inner {
        let mut m: Inner = Elements.try_into().ok().unwrap();
        let mut total: Inner = Inner::one();
        for _ in 0..n {
            total = total * m;
            m = m - Inner::one();
        }
        total
    }

    /// Calculate the permutation of an array
    pub fn try_calculate(mut arr: [usize; Elements]) -> Option<Self> {
        let mut r: Inner = Inner::zero();
        'outer: for index in (0..Elements) {
            let mut swap_element = arr[index];

            let diff = 'inner: loop {
                match swap_element.checked_sub(index) {
                    None => return None, //Array is invalid
                    Some(0) => continue 'outer,
                    Some(diff) => {
                        swap_element = arr[swap_element];
                        if swap_element == index {
                            break 'inner diff;
                        }
                    }
                }
            };
            let amount = arr[index] - index;

            arr.swap(index, index + diff);

            let slot_multiplier = Self::get_inverse_factorial(index);
            let change = (slot_multiplier * (Inner::try_from(amount).ok().unwrap()));
            r = r + change;
        }
        Some(Self(r))
    }

    pub fn get_old_index(&self, new_index: usize) -> usize {
        debug_assert!(new_index < Elements);
        let mut arr = Self::DEFAULT_ARRAY;
        self.apply(&mut arr);

        arr[new_index]
    }

    pub fn get_new_index(&self, old_index: usize) -> usize {
        debug_assert!(old_index < Elements);
        let mut arr = Self::DEFAULT_ARRAY;
        self.apply(&mut arr);
        get_index_of(&arr, old_index)
    }

    pub const DEFAULT_ARRAY: [usize; Elements] = {
        let mut arr = [0usize; Elements];
        let mut i = 0;
        while i < Elements {
            arr[i] = i;
            i += 1;
        }
        arr
    };

    pub fn get_max() -> Self {
        let inner = Inner::get_factorial(Elements);
        Self(inner - Inner::one())
    }
}

#[cfg(test)]
mod tests {
    use core::ops::Range;
    use std::collections::HashSet;

    use anyhow::Ok;
    use itertools::Itertools;
    use ntest::test_case;

    use crate::{PermutationInner, Permutation};

    #[test]
    pub fn all_possible_orderings_are_unique() -> Result<(), anyhow::Error> {
        let mut set: HashSet<[i32; 4]> = Default::default();

        let range: Range<u8> = PermutationInner::get_permutation_range(4);

        for o in range {
            let ordering: Permutation<u8, 4> = o.into();
            let mut arr = [0, 1, 2, 3];
            ordering.apply(&mut arr);

            println!("{arr:?}");

            let added = set.insert(arr);
            assert!(added);
        }

        assert_eq!(set.len(), 24);

        Ok(())
    }

    #[test]
    pub fn test_calculate() {
        let range: Range<u8> = PermutationInner::get_permutation_range(4);

        for o in range {
            let ordering: Permutation<u8, 4> = o.into();
            let mut arr: [usize; 4] = [0, 1, 2, 3];
            ordering.apply(&mut arr);
            println!("{arr:?}");
            let calculated = Permutation::<u8, 4>::try_calculate(arr).unwrap();

            assert_eq!(ordering, calculated)
        }
    }

    #[test_case(0, "0123")]
    #[test_case(1, "1023")]
    #[test_case(2, "2103")]
    #[test_case(3, "3120")]
    #[test_case(4, "0213")]
    #[test_case(5, "1203")]
    pub fn should_order_correctly(o: u8, expected: &str) -> Result<(), anyhow::Error> {
        let permutation: Permutation<u8, 4> = Permutation(o);

        let mut arr = [0, 1, 2, 3];

        permutation.apply(&mut arr);

        let actual = arr.into_iter().map(|x| x.to_string()).join("");

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    pub fn test_get_new_index() {
        let range: Range<u8> = PermutationInner::get_permutation_range(4);

        for o in range {
            let ordering: Permutation<u8, 4> = o.into();
            let mut arr = [0, 1, 2, 3];
            ordering.apply(&mut arr);

            for old_index in 0..4 {
                let new_index1 = arr
                    .iter()
                    .enumerate()
                    .filter(|(i, x)| x == &&old_index)
                    .next()
                    .unwrap()
                    .0;
                let new_index2 = ordering.get_new_index(old_index);

                assert_eq!(new_index1, new_index2);
            }
        }
    }

    #[test]
    pub fn test_get_old_index() {
        let range: Range<u8> = PermutationInner::get_permutation_range(4);

        for o in range {
            let ordering: Permutation<u8, 4> = o.into();
            let mut arr = [0, 1, 2, 3];
            ordering.apply(&mut arr);

            for new_index in 0..4 {
                let old_index1 = arr[new_index];
                let old_index2 = ordering.get_old_index(new_index);

                assert_eq!(old_index1, old_index2);
            }
        }
    }

    macro_rules! test_max {
        ($name: ident, $inner:ty, $max_elements:tt) => {
            #[test]
            pub fn $name() {
                let permutation = Permutation::<$inner, $max_elements>::get_max();

                // let mut arr = Permutation::<$inner, $max_elements>::DEFAULT_ARRAY;
                // permutation.apply(&mut arr);
                // println!("{arr:?}");

                //This orders arrays like [3,0,1,2] - effectively rotating them
                let index_of_0 = permutation.get_new_index(0);
                assert_eq!(index_of_0, 1);

                let element_at_0 = permutation.get_old_index(0);
                assert_eq!(element_at_0, $max_elements - 1);
            }
        };
    }

    test_max!(test_max_u8, u8, 5);
    test_max!(test_max_u16, u16, 8);
    test_max!(test_max_u32, u32, 12);
    test_max!(test_max_u64, u64, 20);
    test_max!(test_max_u128, u128, 34);
}
