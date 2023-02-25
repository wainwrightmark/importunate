use crate::{inner::Inner, Permutation};

#[must_use]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct SwapsIterator<I: Inner> {
    inner: I,
    count: u8,
}

impl<I: Inner> SwapsIterator<I> {
    pub fn new<const ELEMENTS: usize>(perm: &Permutation<I, ELEMENTS>) -> Self {
        Self {
            inner: perm.0,
            count: ELEMENTS.try_into().ok().unwrap(),
        }
    }
}

impl<I: Inner> Iterator for SwapsIterator<I> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.is_zero() {
            return None;
        }

        let (r, diff) = self.inner.div_rem(&self.count.into());
        self.inner = r;
        self.count -= 1;
        diff.try_into().ok()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let rem = self.count.into();
        (rem, Some(rem))
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.count.into()
    }
}
