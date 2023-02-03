use crate::{inner::Inner, Permutation};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct SwapsIterator<I: Inner> {
    inner: I,
    count: u8,
}

impl<I: Inner> SwapsIterator<I> {
    pub fn new<const Elements: usize> (perm: &Permutation<I, Elements>) -> Self {
        Self {
            inner: perm.0,
            count: Elements.try_into().ok().unwrap(),
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
        self.count = self.count - 1;
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
