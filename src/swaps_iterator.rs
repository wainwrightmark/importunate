use crate::{inner::Inner, Permutation};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct SwapsIterator<I: Inner, const Elements: usize> {
    inner: I,
    remaining: I,
}

impl<I: Inner, const Elements: usize> SwapsIterator<I, Elements> {
    pub fn new(perm: &Permutation<I, Elements>) -> Self {
        Self {
            inner: perm.0,
            remaining: Elements.try_into().ok().unwrap(),
        }
    }
}

impl<I: Inner, const Elements: usize> Iterator for SwapsIterator<I, Elements> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.is_zero() {
            return None;
        }

        let (r, diff) = self.inner.div_rem(&self.remaining);
        self.inner = r;
        self.remaining = self.remaining.sub(I::one());
        diff.try_into().ok()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let rem = self.remaining.try_into().ok().unwrap();
        (rem, Some(rem))
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.remaining.try_into().ok().unwrap()
    }
}
