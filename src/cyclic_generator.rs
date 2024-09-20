use crate::{inner::Inner, Permutation};

/// An iterator of the cyclic group generated by a particular permutation.
/// The length of this iterator will be the least common multiple of the lengths of the operators cycles
#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[must_use]
pub struct CyclicGenerator<I: Inner, const ELEMENTS: usize> {
    operator: Permutation<I, ELEMENTS>,
    next: Option<Permutation<I, ELEMENTS>>,
}

impl<I: Inner, const ELEMENTS: usize> From<Permutation<I, ELEMENTS>>
    for CyclicGenerator<I, ELEMENTS>
{
    fn from(perm: Permutation<I, ELEMENTS>) -> Self {
        Self {
            operator: perm,
            next: Some(perm),
        }
    }
}

impl<I: Inner, const ELEMENTS: usize> Iterator for CyclicGenerator<I, ELEMENTS> {
    type Item = Permutation<I, ELEMENTS>;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.next?;

        if n.is_default() {
            self.next = None;
        } else {
            self.next = Some(n.combine(&self.operator));
        }

        Some(n)
    }
}
