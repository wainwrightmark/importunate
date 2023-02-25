use core::marker::PhantomData;

use crate::{inner::Inner, Permutation};

/// Decomposes permutations into disjoint cycles.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[must_use]
pub struct Decomposer<I: Inner, const ELEMENTS: usize>{
    array: [u8;ELEMENTS],
    index: u8,
    phantom: PhantomData<I>
}

impl<I: Inner, const ELEMENTS: usize> Iterator for Decomposer<I, ELEMENTS> {
    type Item = Permutation<I, ELEMENTS>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(x) = self.array.get(self.index as usize) {
            let mut x1 = *x;
            let mut i = self.index;
            self.index+=1;
            if x1 != i{
                let mut arr = Permutation::<I, ELEMENTS>::DEFAULT_ARRAY;

                while i != x1 {
                    arr[i as usize] = x1;
                    self.array[i as usize] = i;
                    i = x1;
                    x1 =self.array[i as usize];
                }
                let perm = Permutation::<I, ELEMENTS>::calculate_unchecked(arr, |x|*x);
                return Some(perm);
            }


        }
        return None;
    }
}

impl<I: Inner, const ELEMENTS: usize> From<Permutation<I, ELEMENTS>> for Decomposer<I, ELEMENTS> {
    fn from(perm: Permutation<I, ELEMENTS>) -> Self {
        Self { array: perm.get_array(), index: 0, phantom: Default::default() }
    }
}

