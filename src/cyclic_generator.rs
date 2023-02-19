use crate::{inner::Inner, Permutation};





/// A permutation of a fixed length array
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct CyclicGenerator<I: Inner, const ELEMENTS: usize>{
    operator: Permutation<I, ELEMENTS>,
    next: Option<Permutation<I, ELEMENTS>>
}

impl<I: Inner, const ELEMENTS: usize> CyclicGenerator<I, ELEMENTS> {
    pub fn new(perm: Permutation<I, ELEMENTS>)-> Self{
        Self { operator: perm, next: Some(perm) }
    }
}


impl<I: Inner, const ELEMENTS: usize> Iterator for CyclicGenerator<I, ELEMENTS> {
    type Item = Permutation<I, ELEMENTS>;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(n) = self.next
        else {return None;};

        if n.is_default(){
            self.next = None;
        }else{
            self.next = Some(n.combine(&self.operator))
        }

        Some(n)
    }
}
