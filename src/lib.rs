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
// index of element
// documentation

pub mod inner;

use core::fmt::Debug;
use core::hash::Hash;
use core::ops::Range;

use inner::Inner;
use num::{Integer, Unsigned};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use static_assertions::const_assert;

/// A permutation
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Permutation<I: Inner, const ELEMENTS: usize>(I);

impl<I: Inner, const Elements: usize> From<I> for Permutation<I, Elements> {
    fn from(value: I) -> Self {
        debug_assert!(Elements <= I::MAX_ELEMENTS);
        Self(value)
    }
}

impl<I: Inner, const Elements: usize> Default for Permutation<I, Elements> {
    fn default() -> Self {
        debug_assert!(Elements <= I::MAX_ELEMENTS);
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

impl<I: Inner, const Elements: usize> Permutation<I, Elements> {
    /// Apply this permutation to an array, reordering the first `Elements` elements
    pub fn apply<T>(&self, arr: &mut [T]) {
        let mut rem = self.0;
        let len = arr.len().min(Elements);

        for i in (0..len) {
            if rem.is_zero() {
                return;
            }
            let (r, diff) = rem.div_rem(&(len - i).try_into().ok().unwrap());
            rem = r;
            arr.swap(i, (diff.try_into().ok().unwrap() + i));
        }
    }

    /// The range of all possible permutations of this number of elements
    pub fn all() -> impl Iterator<Item = Self> {
        let range = I::get_permutation_range(Elements);
        range.map(|x| Self(x))
    }
    fn get_swaps(&self) -> [usize; Elements] {
        let mut swaps = [0; Elements];

        let mut rem = self.0;

        for i in (0..Elements) {
            if rem.is_zero() {
                break;
            }
            let (r, diff) = rem.div_rem(&(Elements - i).try_into().ok().unwrap());
            rem = r;
            swaps[i] = (diff.try_into().ok().unwrap());
        }
        swaps
    }

    fn from_swaps(swaps: &[usize; Elements]) -> Self {
        let mut inner: I = I::zero();
        let mut mult: I = I::one();
        for i in 0..Elements {
            let r = mult * swaps[i].try_into().ok().unwrap();
            inner = inner + r;
            mult = mult * (Elements - i).try_into().ok().unwrap();
        }

        Self(inner)
    }

    //e.g. 1,4,12,24 or 1,5,20,60,120
    fn get_backtorial(n: usize) -> I {
        let mut m: I = Elements.try_into().ok().unwrap();
        let mut total: I = I::one();
        for _ in 0..n {
            total = total * m;
            m = m - I::one();
        }
        total
    }

    fn test_unique(arr: impl Iterator<Item = usize>) -> bool {
        let mut test = 0u64;

        for x in arr.take(Elements) {
            test = test | 1 << x;
        }

        let r = test.count_ones() as usize == Elements;
        r
    }

    pub fn calculate<T, F: Fn(&T) -> usize>(mut arr: [T; Elements], f: F) -> Self {
        debug_assert!(Self::test_unique(arr.iter().map(|x| f(x))));
        let mut slot_multiplier: I = I::one();
        let mut inner: I = I::zero();
        'outer: for index in (0..Elements) {
            let mut swap_element = f(&arr[index]);

            'inner: loop {
                match swap_element.checked_sub(index) {
                    None => unreachable!(), //Array is invalid
                    Some(0) => break 'inner,
                    Some(diff) => {
                        swap_element = f(&arr[swap_element]);
                        if swap_element == index {
                            let amount = f(&arr[index]) - index;

                            arr.swap(index, index + diff);

                            let change = (slot_multiplier * (I::try_from(amount).ok().unwrap()));
                            inner = inner + change;
                            break 'inner;
                        }
                    }
                }
            }
            slot_multiplier = slot_multiplier * I::try_from((Elements - index)).ok().unwrap();
        }
        Self(inner)
    }

    /// Calculate the permutation of an array
    /// Beware: if the array contains duplicate elements, this may loop forever
    pub fn try_calculate<T, F: Fn(&T) -> usize>(mut arr: [T; Elements], f: F) -> Option<Self> {
        if !Self::test_unique(arr.iter().map(|x| f(x))) {
            return None;
        }
        Some(Self::calculate(arr, f))
    }

    pub fn element_at_index<T, F: Fn(usize) -> T>(&self, new_index: usize, f: F) -> T {
        debug_assert!(new_index < Elements);
        let mut current_index = new_index;
        let mut fact: I = Self::get_backtorial(current_index);
        let mut rem: I = self.0;

        let mut watch_index = if current_index + 1 == Elements {
            current_index
        } else {
            let (diff, rem_new) = rem.div_rem(&fact);
            rem = rem_new;
            let diff = diff % (Elements - current_index).try_into().ok().unwrap();
            let diff = diff.try_into().ok().unwrap();
            current_index + diff
        };

        while let Some(ci) = current_index.checked_sub(1) {
            current_index = ci;
            fact = fact / (Elements - current_index).try_into().ok().unwrap();
            let (diff, rem_new) = rem.div_rem(&fact);
            rem = rem_new;
            let diff = diff.try_into().ok().unwrap();
            if current_index + diff == watch_index {
                watch_index = current_index;
            }
        }
        f(watch_index)
    }

    pub fn index_of<T, F: Fn(&T) -> usize>(&self, element: &T, f: F) -> usize {
        //TODO more efficient implementation
        let old_index = f(element);
        debug_assert!(old_index < Elements);
        let arr = self.get_array();
        get_index_of(&arr, old_index)
    }

    const DEFAULT_ARRAY: [usize; Elements] = {
        let mut arr = [0usize; Elements];
        let mut i = 0;
        while i < Elements {
            arr[i] = i;
            i += 1;
        }
        arr
    };

    pub fn get_array(&self) -> [usize; Elements] {
        let mut arr = Self::DEFAULT_ARRAY;
        self.apply(&mut arr);
        arr
    }

    pub fn combine(&self, rhs: &Self) -> Self {
        let mut arr = self.get_array();
        rhs.apply(&mut arr);
        let r = Self::calculate(arr, |&x| x);
        r
    }

    // pub fn combine2(&self, rhs: &Self)-> Self{
    //     let mut arr = self.get_array();
    //     rhs.apply(&mut arr);
    //     let r = Self::try_calculate(arr, |&x|x).unwrap();
    //     r
    // }

    pub fn invert(&self) -> Self {
        let mut swaps = self.get_swaps();
        let mut updated = 0;

        for i in (0..Elements).rev() {
            let swap = swaps[i];
            if swap != 0 {
                let i2 = i + swap;
                for j in (0..i).rev() {
                    if swaps[j] + j == i {
                        swaps[j] = i2 - j;
                        updated += 1;
                    } else if swaps[j] + j == i2 {
                        swaps[j] = i - j;
                        updated += 1;
                    }
                }
            }
        }

        if updated == 0 {
            return self.clone();
        }

        Self::from_swaps(&swaps)

        /*
        [0, 0, 0, 0] [0, 0, 0, 0]
        [1, 0, 0, 0] [1, 0, 0, 0]
        [2, 0, 0, 0] [2, 0, 0, 0]
        [3, 0, 0, 0] [3, 0, 0, 0]
        [0, 1, 0, 0] [0, 1, 0, 0]
        [1, 1, 0, 0] [2, 1, 0, 0] *
        [2, 1, 0, 0] [1, 1, 0, 0] *
        [3, 1, 0, 0] [3, 1, 0, 0]
        [0, 2, 0, 0] [0, 2, 0, 0]
        [1, 2, 0, 0] [3, 2, 0, 0] *
        [2, 2, 0, 0] [2, 2, 0, 0]
        [3, 2, 0, 0] [1, 2, 0, 0] *
        [0, 0, 1, 0] [0, 0, 1, 0]
        [1, 0, 1, 0] [1, 0, 1, 0]
        [2, 0, 1, 0] [3, 0, 1, 0] *
        [3, 0, 1, 0] [2, 0, 1, 0] *
        [0, 1, 1, 0] [0, 2, 1, 0] *
        [1, 1, 1, 0] [3, 2, 1, 0] *
        [2, 1, 1, 0] [1, 2, 1, 0] *
        [3, 1, 1, 0] [2, 2, 1, 0] *
        [0, 2, 1, 0] [0, 1, 1, 0] *
        [1, 2, 1, 0] [2, 1, 1, 0] *
        [2, 2, 1, 0] [3, 1, 1, 0] *
        [3, 2, 1, 0] [1, 1, 1, 0] *
                                         */
    }

    pub fn get_max() -> Self {
        let inner = I::get_factorial(Elements);
        Self(inner - I::one())
    }
}

#[cfg(test)]
mod tests {
    use core::ops::Range;
    use std::collections::HashSet;

    use anyhow::Ok;
    use itertools::Itertools;
    use ntest::test_case;

    use crate::{Inner, Permutation};

    #[test]
    pub fn test_swaps() {
        for permutation in Permutation::<u8, 4>::all() {
            let swaps = permutation.get_swaps();
            let ordering2 = Permutation::<u8, 4>::from_swaps(&swaps);

            assert_eq!(permutation, ordering2)
        }
    }

    #[test]
    pub fn test_combine() {
        for p_left in Permutation::<u8, 4>::all() {
            for p_right in Permutation::<u8, 4>::all() {
                let combined = p_left.combine(&p_right);

                println!(
                    "{:>2} + {:>2} = {:>2}; {:?} + {:?} = {:?}",
                    p_left.0,
                    p_right.0,
                    combined.0,
                    p_left.get_swaps(),
                    p_right.get_swaps(),
                    combined.get_swaps()
                );
            }
        }
    }

    #[test]
    pub fn test_invert() {
        for permutation in Permutation::<u8, 4>::all() {
            let arr = permutation.get_array();
            let inverse = permutation.invert();
            let inverse_arr = inverse.get_array();

            println!(
                "{}:  {:?} {:?}",
                permutation.0,
                permutation.get_swaps(),
                inverse.get_swaps()
            );

            let mut arr2 = arr.clone();
            inverse.apply(&mut arr2);
            let new_perm = Permutation::<u8, 4>::try_calculate(arr2, |&x| x).unwrap();

            assert_eq!(0, new_perm.0)
        }
    }

    #[test]
    pub fn test_element_at_index() {
        for perm in Permutation::<u8, 4>::all() {
            let mut arr = [0, 1, 2, 3];
            perm.apply(&mut arr);

            println!("");
            println!("");
            println!("{arr:?} :: {}", perm.0);
            let mut arr2 = [0, 1, 2, 3];

            for index in 0..4 {
                println!("");
                println!("Index: {index}");
                let element = perm.element_at_index(index, |x| x);
                println!("Element: {element}");
                arr2[index] = element;
            }

            assert_eq!(arr, arr2);
        }
    }

    #[test]
    pub fn test_index_of() {
        for perm in Permutation::<u8, 4>::all() {
            let mut arr = [0, 1, 2, 3];
            perm.apply(&mut arr);

            let mut arr1 = [0, 1, 2, 3];
            let mut arr2 = [0, 1, 2, 3];

            for index in 0..4usize {
                arr1[index] = arr
                    .iter()
                    .enumerate()
                    .filter(|(i, x)| x == &&index)
                    .next()
                    .unwrap()
                    .0;

                arr2[index] = perm.index_of(&index, |&x| x);
            }

            assert_eq!(arr1, arr2)
        }
    }

    #[test]
    pub fn all_possible_orderings_are_unique() -> Result<(), anyhow::Error> {
        let mut set: HashSet<[usize; 4]> = Default::default();

        for perm in Permutation::<u8, 4>::all() {
            let arr = perm.get_array();
            println!("{arr:?}");

            let added = set.insert(arr);
            assert!(added);
        }

        assert_eq!(set.len(), 24);

        Ok(())
    }

    #[test]
    pub fn test_calculate() {
        for perm in Permutation::<u8, 4>::all() {
            let arr = perm.get_array();
            println!("{arr:?}");
            let calculated = Permutation::<u8, 4>::try_calculate(arr, |x| *x).unwrap();

            assert_eq!(perm, calculated)
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

    macro_rules! test_max {
        ($name: ident, $inner:ty, $max_elements:tt) => {
            #[test]
            pub fn $name() {
                let permutation = Permutation::<$inner, $max_elements>::get_max();

                // let mut arr = Permutation::<$inner, $max_elements>::DEFAULT_ARRAY;
                // permutation.apply(&mut arr);
                // println!("{arr:?}");

                //This orders arrays like [3,0,1,2] - effectively rotating them
                let index_of_0 = permutation.index_of(&0, |&x| x);
                assert_eq!(index_of_0, 1);

                let element_at_0 = permutation.element_at_index(0, |x| x);
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
