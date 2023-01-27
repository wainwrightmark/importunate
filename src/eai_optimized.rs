//Version optimized for Element At Index

//Produces the following output
/*
[0, 1, 2, 3]
[1, 0, 2, 3]
[2, 1, 0, 3]
[3, 1, 2, 0]
[0, 2, 1, 3]
[1, 2, 0, 3]
[2, 0, 1, 3]
[3, 2, 1, 0]
[0, 3, 2, 1]
[1, 3, 2, 0]
[2, 3, 0, 1]
[3, 0, 2, 1]
[0, 1, 3, 2]
[1, 0, 3, 2]
[2, 1, 3, 0]
[3, 1, 0, 2]
[0, 2, 3, 1]
[1, 2, 3, 0]
[2, 0, 3, 1]
[3, 2, 0, 1]
[0, 3, 1, 2]
[1, 3, 0, 2]
[2, 3, 1, 0]
[3, 0, 1, 2]
 */

/*
impl<Inner: PermuationInner, const Elements: usize> Permutation<Inner, Elements> {
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

    pub fn get_inverse_factorial(n : usize)-> Inner{
        let mut m : Inner = Elements.try_into().ok().unwrap();
        let mut total: Inner = Inner::one();
        for _ in 0..n{
            total = total * m;
            m = m- Inner::one();
        }
        total
    }

    /// Calculate the permutation of an array
    pub fn try_calculate(mut arr: [usize; Elements]) -> Option<Self> {
        let mut r: Inner = Inner::zero();
        'outer: for index in (0..Elements) {
            let mut swap_element = arr[index];

            let diff = 'inner: loop{
                match swap_element.checked_sub(index) {
                    None => return None, //Array is invalid
                    Some(0) => continue 'outer,
                    Some(diff) => {
                        swap_element = arr[swap_element];
                        if swap_element == index{
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
    } */