
//Use the code her of index - of - element optimized version

/*
Produces output in this order - cheaper to calculate index of element
[0, 1, 2, 3]
[0, 1, 3, 2]
[0, 2, 1, 3]
[0, 3, 1, 2]
[0, 3, 2, 1]
[0, 2, 3, 1]
[1, 0, 2, 3]
[1, 0, 3, 2]
[2, 0, 1, 3]
[3, 0, 1, 2]
[3, 0, 2, 1]
[2, 0, 3, 1]
[2, 1, 0, 3]
[3, 1, 0, 2]
[1, 2, 0, 3]
[1, 3, 0, 2]
[2, 3, 0, 1]
[3, 2, 0, 1]
[3, 1, 2, 0]
[2, 1, 3, 0]
[3, 2, 1, 0]
[2, 3, 1, 0]
[1, 3, 2, 0]
[1, 2, 3, 0]
 */

// pub fn apply<T>(&self, arr: &mut [T]) {
//     let mut rem = self.0;
//     let len = arr.len().min(Elements);

//     for i in (0..len).rev() {
//         let (r, diff) = rem.divmod(len - i);
//         rem = r;
//         arr.swap(i, diff + i);
//     }
// }

// /// Calculate the permutation of an array
// pub fn try_calculate(mut arr: [usize; Elements]) -> Option<Self> {
//     let mut r: Inner = Inner::zero();
//     'outer: for index in (0..Elements) {
//         let mut swap_element = arr[index];

//         let diff = 'inner: loop{
//             match swap_element.checked_sub(index) {
//                 None => return None, //Array is invalid
//                 Some(0) => continue 'outer,
//                 Some(diff) => {
//                     swap_element = arr[swap_element];
//                     if swap_element == index{
//                         break 'inner diff;
//                     }
//                 }
//             }
//         };

//         arr.swap(index, index + diff);

//         let slot_multiplier = Inner::get_factorial(Elements - (index + 1));
//         let change = (slot_multiplier * (Inner::try_from(diff).ok().unwrap()));
//         r = r + change;
//     }
//     Some(Self(r))
// }