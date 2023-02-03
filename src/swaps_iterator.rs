use crate::{inner::Inner, Permutation};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct SwapsIterator<I: Inner, const Elements: usize> {
    rem: I,
    multiplier: I,
}

impl<I: Inner, const Elements: usize> SwapsIterator<I, Elements> {
    pub fn new(perm: &Permutation<I, Elements>)-> Self{
        Self { rem: perm.0, multiplier: Elements.try_into().ok().unwrap() }
    }
}



impl<I: Inner, const Elements: usize> Iterator for SwapsIterator<I, Elements> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rem.is_zero() {
            return None;
        }

        let (r, diff) = self.rem.div_rem(&self.multiplier);
        self.rem = r;
        self.multiplier = self.multiplier.sub(I::one());
        diff.try_into().ok()
    }
}
