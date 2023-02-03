#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![doc(html_root_url = "https://docs.rs/importunate/0.4.0")]
#![deny(missing_docs)]
#![deny(warnings, dead_code, unused_imports, unused_mut)]
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
//! fn main() {
//! let arr1 = [2,0,1,3];
//! let mut arr2 = ["zero", "one", "two", "three"];
//! let perm = Permutation::<u8,4>::calculate_unchecked(arr1, |&x|x);
//! perm.apply(&mut arr2);
//!
//! assert_eq!(arr2,["two","zero", "one",  "three"] );
//! }
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
// documentation
// optional rand
// errors

/// Inner types that Permutations can use
pub mod inner;
mod swaps_iterator;

use core::hash::Hash;
use core::{cmp::Ordering, fmt::Debug};

use inner::Inner;
#[cfg(any(test, feature = "serde"))]
use serde::{Deserialize, Serialize};

/// A permutation of a fixed length array
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(any(test, feature = "serde"), derive(Serialize), serde(transparent))]
pub struct Permutation<I: Inner, const ELEMENTS: usize>(I);

#[cfg(any(test, feature = "serde"))]
impl<'de, I: Inner + Deserialize<'de>, const ELEMENTS: usize> Deserialize<'de>
    for Permutation<I, ELEMENTS>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        debug_assert!(ELEMENTS <= I::MAX_ELEMENTS);
        let i = I::deserialize(deserializer)?;
        if i > Self::get_last().0 {
            return Err(serde::de::Error::custom(format!(
                "number out of range: {:?}",
                i
            )));
        }

        Ok(Self(i))
    }
}

#[cfg(feature = "arbitrary")]
use arbitrary::Arbitrary;
use swaps_iterator::SwapsIterator;
#[cfg(feature = "arbitrary")]
impl<'a, I: Inner, const ELEMENTS: usize> Arbitrary<'a> for Permutation<I, ELEMENTS> {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        debug_assert!(ELEMENTS <= I::MAX_ELEMENTS);
        let bytes = u.bytes(Self::REQUIRED_BYTES)?;

        let inner = I::from_le_byte_array(bytes);
        let inner = inner.mod_floor(&Self::get_last().0);
        Ok(Self(inner))
    }

    fn arbitrary_take_rest(mut u: arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        debug_assert!(ELEMENTS <= I::MAX_ELEMENTS);
        Self::arbitrary(&mut u)
    }

    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        let _ = depth;
        (Self::REQUIRED_BYTES, Some(Self::REQUIRED_BYTES))
    }
}

impl<I: Inner, const ELEMENTS: usize> From<I> for Permutation<I, ELEMENTS> {
    fn from(value: I) -> Self {
        debug_assert!(ELEMENTS <= I::MAX_ELEMENTS);
        Self(value)
    }
}

impl<I: Inner, const ELEMENTS: usize> Default for Permutation<I, ELEMENTS> {
    fn default() -> Self {
        debug_assert!(ELEMENTS <= I::MAX_ELEMENTS);
        Self(Default::default())
    }
}


impl<I: Inner, const ELEMENTS: usize> Permutation<I, ELEMENTS> {

    #[must_use]
    /// The inner value of this permutation
    pub fn inner(&self) -> I {
        self.0
    }

    /// Apply this permutation to an array, reordering the first `ELEMENTS` elements
    pub fn apply<T>(&self, arr: &mut [T]) {
        for (i, swap) in self.swaps().enumerate() {
            arr.swap(i, usize::from(swap) + i);
        }
    }
    /// Apply the inverse of this permutation to an array, reordering the first `ELEMENTS` elements
    pub fn apply_inverse<T>(&self, arr: &mut [T]) {
        for (i, swap) in self.swaps_array().into_iter().enumerate().rev() {
            arr.swap(i, usize::from(swap) + i);
        }
    }

    #[must_use]
    /// The range of all possible permutations of this number of elements
    pub fn all() -> impl Iterator<Item = Self> {
        let range = I::get_permutation_range(ELEMENTS);
        range.map(|x| Self(x))
    }

    #[must_use]
    fn swaps(&self) -> SwapsIterator<I> {
        SwapsIterator::new(self)
    }

    #[must_use]
    fn swaps_array(&self) -> [u8; ELEMENTS] {
        let mut swaps = [0; ELEMENTS];

        for (i, swap) in self.swaps().enumerate() {
            swaps[i] = swap;
        }
        swaps
    }

    #[must_use]
    fn from_swaps(swaps: impl Iterator<Item = u8>) -> Self {
        let mut inner: I = I::zero();
        let mut mult: I = I::one();

        for (i, swap) in swaps.enumerate() {
            let r = mult * swap.try_into().ok().unwrap();
            inner = inner + r;
            mult = mult * (ELEMENTS - i).try_into().ok().unwrap();
        }

        Self(inner)
    }

    #[must_use]
    fn test_unique(iterator: impl Iterator<Item = usize>) -> bool {
        let mut test = 0u64;

        for x in iterator.take(ELEMENTS) {
            test = test | 1 << x;
        }

        let r = test.count_ones() as usize == ELEMENTS;
        r
    }

    #[must_use]
    /// Calculate the permutation for any list, even one containing duplicates.
    /// There is a performance penalty for using this - it will make n * n comparisons
    pub fn calculate_incomplete<T: Ord>(slice: &[T]) -> Self {
        let mut arr = Self::DEFAULT_ARRAY;

        for (index, element) in slice.iter().take(ELEMENTS).enumerate() {
            let mut c = 0;
            for (jindex, el) in slice.iter().take(ELEMENTS).enumerate() {
                match element.cmp(el) {
                    Ordering::Greater => c += 1,
                    Ordering::Equal => {
                        if index > jindex {
                            c += 1;
                        }
                    }
                    Ordering::Less => {}
                }
            }
            arr[index] = c;
        }

        Self::calculate_unchecked(arr, |&x| x)
    }

    /// Calculate the permutation of an array.
    /// This will panic or loop forever if the arrays elements contain duplicates or elements outsize `0..ELEMENTS`
    #[must_use]
    pub fn calculate_unchecked<T, F: Fn(&T) -> usize>(mut arr: [T; ELEMENTS], f: F) -> Self {
        debug_assert!(Self::test_unique(arr.iter().map(|x| f(x))));
        let mut slot_multiplier: I = I::one();
        let mut inner: I = I::zero();
        for index in 0..ELEMENTS {
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

                            let change = slot_multiplier * (I::try_from(amount).ok().unwrap());
                            inner = inner + change;
                            break 'inner;
                        }
                    }
                }
            }
            slot_multiplier = slot_multiplier * I::try_from(ELEMENTS - index).ok().unwrap();
        }
        Self(inner)
    }

    #[must_use]
    /// Calculate the permutation of an array
    /// This will return `None` if the array's elements contain duplicates or elements outsize `0..ELEMENTS`
    pub fn try_calculate<T, F: Fn(&T) -> usize>(arr: [T; ELEMENTS], f: F) -> Option<Self> {
        if !Self::test_unique(arr.iter().map(|x| f(x))) {
            return None;
        }
        Some(Self::calculate_unchecked(arr, f))
    }

    #[must_use]
    /// Get the element at the given index of the permutation
    pub fn element_at_index<T, F: Fn(usize) -> T>(&self, new_index: usize, f: F) -> T {
        debug_assert!(new_index < ELEMENTS);

        let mut swaps = [0; ELEMENTS];

        for (i, swap) in self.swaps().enumerate().take(new_index + 1) {
            swaps[i] = swap;
        }

        let old_index = Self::element_at_index_from_swaps(&swaps, new_index as u8);

        f(old_index.into())
    }

    #[must_use]
    fn element_at_index_from_swaps(swaps: &[u8], index: u8) -> u8 {
        let mut current = swaps[usize::from(index)] + index;

        for j in (0..index).rev() {
            if swaps[usize::from(j)] + j == current {
                current = j;
            }
        }
        current
    }

    #[must_use]
    /// Get the new index of the given element from the permutation
    pub fn index_of<T, F: Fn(&T) -> u8>(&self, element: &T, f: F) -> u8 {
        let old_index = f(element);
        debug_assert!(usize::from(old_index) < ELEMENTS);

        Self::index_of_element_from_swaps(self.swaps(), old_index)
    }

    fn index_of_element_from_swaps(swaps_iter: impl Iterator<Item = u8>, mut index: u8) -> u8 {
        for (j, diff) in swaps_iter.enumerate() {
            let j = j as u8;
            match j.cmp(&index) {
                Ordering::Less => {
                    if j + diff == index {
                        return j.into();
                    }
                }
                Ordering::Equal => {
                    index = index + diff;
                }
                Ordering::Greater => {
                    break;
                }
            }
        }
        return index;
    }

    const DEFAULT_ARRAY: [usize; ELEMENTS] = {
        let mut arr = [0usize; ELEMENTS];
        let mut i = 0;
        while i < ELEMENTS {
            arr[i] = i;
            i += 1;
        }
        arr
    };

    #[must_use]
    /// Get the complete array of this permutation's elements
    pub fn get_array(&self) -> [usize; ELEMENTS] {
        let mut arr = Self::DEFAULT_ARRAY;
        self.apply(&mut arr);
        arr
    }


    #[must_use]
    /// Write this permutation to a byte array
    /// Panics if `BYTES` is too small for permutations of this many elements
    /// See `REQUIRED_BYTES`
    pub fn to_le_byte_array<const BYTES: usize>(&self) -> [u8; BYTES] {
        assert!(BYTES >= Self::REQUIRED_BYTES);

        self.0.to_le_byte_array()
    }


    #[must_use]
    /// Read this permutation from a byte array
    /// Panics if `BYTES` is too small for permutations of this many elements
    /// See `REQUIRED_BYTES`
    pub fn try_from_le_byte_array(bytes: &[u8]) -> Option<Self> {
        assert!(bytes.len() >= Self::REQUIRED_BYTES);

        let inner = I::from_le_byte_array(bytes);
        if inner <= Self::get_last().0 {
            Self(inner).into()
        } else {
            None
        }
    }

    /// The number of bytes required to store a permutation of this many elements
    pub const REQUIRED_BYTES: usize = {
        match ELEMENTS {
            0..=5 => 1,
            ..=8 => 2,
            ..=10 => 3,
            ..=12 => 4,
            ..=14 => 5,
            ..=16 => 6,
            ..=18 => 7,
            ..=20 => 8,
            ..=22 => 9,
            ..=24 => 10,
            ..=25 => 11,
            ..=27 => 12,
            ..=29 => 13,
            ..=30 => 14,
            ..=32 => 15,
            ..=34 => 16,
            _ => {
                panic!("ELEMENTS is too big")
            }
        }
    };

    #[must_use]
    /// Invert this permutation
    /// This produces the permutation that will reorder the array back to its original order
    pub fn invert(&self) -> Self {
        let mut swaps = self.swaps_array();
        let mut is_different = false;

        for i in (0..(ELEMENTS as u8)).rev() {
            let swap: u8 = swaps[usize::from(i)];
            if swap != 0 {
                let i2 = i + swap;
                for j in (0..i).rev() {
                    if swaps[usize::from(j)] + j == i {
                        swaps[usize::from(j)] = i2 - j;
                        is_different = true;
                    } else if swaps[usize::from(j)] + j == i2 {
                        swaps[usize::from(j)] = i - j;
                        is_different = true;
                    }
                }
            }
        }

        if !is_different { //A great many permutations are their own inverses, so we can skip calculating if that is the case
            return self.clone();
        }

        Self::from_swaps(swaps.into_iter())
    }

    #[must_use]
    /// Gets the permutation of this many elements with the highest inner value
    pub fn get_last() -> Self {
        let inner = I::get_factorial(ELEMENTS);
        Self(inner - I::one())
    }


    #[must_use]
    /// Combine this permutation with another. Producing a permutation equivalent to performing this and then the other.
    /// Note that this operation is neither commutative nor associative
    pub fn combine(&self, rhs: &Self) -> Self {
        let mut arr = self.get_array();
        rhs.apply(&mut arr);
        let r = Self::calculate_unchecked(arr, |&x| x);
        r
    }

    //The slow (but oh so elegant) version of combine
    // pub fn combine(&self, rhs: &Self) -> Self {
    //     let mut left_swaps = self.swaps_array();
    //     for (i, diff) in rhs.swaps().enumerate() {
    //         Self::swap_swaps(&mut left_swaps[i..], diff);
    //     }

    //     Self::from_swaps(left_swaps.into_iter())
    // }

    // fn swap_swaps(mut swaps_arr: &mut [u8], mut diff: u8) {
    //     while diff > 0 {
    //         let index_of_zero = Self::index_of_element_from_swaps(swaps_arr.into_iter().map(|x|*x), 0);
    //         let element_at_n = Self::element_at_index_from_swaps(swaps_arr, diff);

    //         swaps_arr[0] = element_at_n;
    //         if index_of_zero > 0 {
    //             let min = diff.min(index_of_zero);
    //             diff = diff.abs_diff(index_of_zero);
    //             swaps_arr = &mut swaps_arr[(min as usize)..];
    //             diff = diff;
    //         }
    //         else{
    //             break;
    //         }
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use anyhow::Ok;
    use itertools::Itertools;
    use ntest::test_case;
    use std::collections::HashSet;

    #[test]
    pub fn test_combine() {
        let mut combinations = [Permutation::<u8, 4>::default(); 576];
        let mut i = 0;
        for p_left in Permutation::<u8, 4>::all() {
            for p_right in Permutation::<u8, 4>::all() {
                let combined1 = p_left.combine(&p_right);
                combinations[i] = combined1;
                i += 1;
            }
        }

        insta::assert_snapshot!(combinations
            .map(|x| format!("{:?}", x.swaps_array()))
            .join("\n"))
    }

    #[test]
    pub fn test_calculate_incomplete() {
        let anagram = Permutation::<u16, 7>::calculate_incomplete("anagram".as_bytes());

        let mut to_change = "anagram".bytes().collect_vec();
        anagram.invert().apply(to_change.as_mut_slice());

        let converted = String::from_utf8(to_change).unwrap();

        assert_eq!(converted, "aaagmnr")
    }

    #[test]
    pub fn find_anagrams() {
        let anagram = Permutation::<u16, 7>::calculate_incomplete("anagram".as_bytes());
        let admiral = Permutation::<u16, 7>::calculate_incomplete("admiral".as_bytes());

        assert_eq!(anagram, admiral.invert());

        let anagrams = Permutation::<u16, 7>::calculate_incomplete("anagrams".as_bytes());

        assert_eq!(anagram, anagrams); //extra characters are ignored

        let anagr = Permutation::<u16, 5>::calculate_incomplete("anagr".as_bytes());
        let anag = Permutation::<u16, 5>::calculate_incomplete("anag".as_bytes());

        assert_eq!(anagr, anag); //extra characters are ignored
    }

    #[test]
    pub fn test_bytes() {
        for perm in Permutation::<u32, 10>::all() {
            let bytes: [u8; 3] = perm.to_le_byte_array();

            let perm2 = Permutation::<u32, 10>::try_from_le_byte_array(&bytes).unwrap();
            assert_eq!(perm, perm2)
        }
    }

    #[test]
    pub fn test_swaps() {
        for permutation in Permutation::<u8, 4>::all() {
            let swaps = permutation.swaps_array();
            let ordering2 = Permutation::<u8, 4>::from_swaps(swaps.into_iter());

            assert_eq!(permutation, ordering2)
        }
    }

    #[test]
    pub fn test_invert() {
        for permutation in Permutation::<u8, 4>::all() {
            let arr = permutation.get_array();
            let inverse = permutation.invert();

            println!(
                "{}:  {:?} {:?}",
                permutation.0,
                permutation.swaps_array(),
                inverse.swaps_array()
            );

            let mut arr2 = arr.clone();
            inverse.apply(&mut arr2);
            let new_perm = Permutation::<u8, 4>::try_calculate(arr2, |&x| x).unwrap();

            assert_eq!(0, new_perm.0)
        }
    }

    #[test]
    pub fn test_apply_inverse() {
        for permutation in Permutation::<u8, 4>::all() {
            let mut arr = permutation.get_array();
            permutation.apply_inverse(&mut arr);

            assert_eq!(arr, Permutation::<u8, 4>::DEFAULT_ARRAY)
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

            for index in 0..4u8 {
                arr1[index as usize] = arr
                    .iter()
                    .enumerate()
                    .filter(|(_, x)| x == &&index)
                    .next()
                    .unwrap()
                    .0 as u8;

                arr2[index as usize] = perm.index_of(&index, |&x| x as u8);
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
                let permutation = Permutation::<$inner, $max_elements>::get_last();

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

    #[test]
    fn test_ser_de() {
        use serde_test::*;
        let perm = Permutation::<u8, 4>::calculate_incomplete(&[2, 0, 1, 3]);

        assert_tokens(&perm, &[Token::U8(6)])
    }
}
