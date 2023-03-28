use crate::{inner::Inner, Permutation};

/// Can be used to solve a permutation, finding the the shortest combination of a fixed set of permutations that leads to it

pub struct SolveContext<I: Inner, const ELEMENTS: usize> {
    /// Every index contains four pairs of bits
    /// Each permutation is associated with a pair
    /// Permutation p is associated with pair (p % 4) at index (p / 4)
    /// The meanings of the pairs
    /// 00: p is 0 mod 3 steps from solved
    /// 01: p is 1 mod 3 steps from solved
    /// 10: p is 2 mod 3 steps from solved
    /// 11: p cannot be solved with this set of moves
    vec: Vec<u8>,

    /// The total number of solvable permutations
    pub number_solvable: I,

    moves: Vec<Permutation<I, ELEMENTS>>,
}

impl<I: Inner, const ELEMENTS: usize> SolveContext<I, ELEMENTS> {
    /// Try to Deconstruct the inverse of this permutation into the shortest sequence of permutations from the allowed list
    pub fn solve(
        &self,
        mut perm: Permutation<I, ELEMENTS>,
    ) -> Option<Vec<Permutation<I, ELEMENTS>>> {
        let mut result = vec![];
        let mut moves_mod_3 = self.get_bits(perm);
        if moves_mod_3 == 3 {
            return None;
        }

        'outer: while !perm.is_default() {
            let next = (moves_mod_3 + 2) % 3;

            for m in &self.moves {
                let combined = perm.combine(m);
                let mm3 = self.get_bits(combined);
                if mm3 == next {
                    perm = combined;
                    result.push(*m);
                    moves_mod_3 = next;
                    continue 'outer;
                }
            }

            unreachable!()
        }

        Some(result)
    }

    fn get_bits(&self, perm: Permutation<I, ELEMENTS>) -> u8 {
        let us: usize = perm.0.try_into().unwrap_or_else(|_| unreachable!());
        let index = us / 4usize;

        let shift = (us % 4) * 2;
        3u8 & (self.vec[index] >> shift)
    }

    fn combine_arrays(lhs: &[u8; ELEMENTS], rhs_swaps: &[u8; ELEMENTS]) -> [u8; ELEMENTS] {
        let mut result = *lhs;

        for (index, &swap) in rhs_swaps.iter().enumerate() {
            result.swap(index, index + usize::from(swap));
        }
        result
    }

    /// Create a new solver from a fixed set of moves. This will also use the inverses of those moves
    /// # Panics
    ///
    /// This will panic if the number of possible solutions is greater than `usize::MAX`
    #[must_use]
    pub fn new(mut moves: Vec<Permutation<I, ELEMENTS>>) -> Self {
        let Ok(total) = I::get_factorial(ELEMENTS).try_into() else{
            panic!("Cannot solve for {ELEMENTS} elements!");
        };

        for m in moves.clone().iter(){
            moves.push(m.invert());
        }
        moves.sort();
        moves.dedup();

        let mut vec = vec![u8::MAX; total];

        let mut number_solvable = 0;
        let current: &mut Vec<[u8; ELEMENTS]> =
            &mut vec![Permutation::<I, ELEMENTS>::default().get_array()];
        let next: &mut Vec<[u8; ELEMENTS]> = &mut vec![];

        let move_swaps: Vec<_> = moves.iter().map(Permutation::swaps_array).collect();

        let mut moves_mod_3 = 0;
        while !current.is_empty() && number_solvable <= total {
            for perm_arr in current.drain(..) {
                let perm1 = Permutation::<I, ELEMENTS>::calculate_unchecked(perm_arr, |x| *x);

                let us: usize = perm1.0.try_into().unwrap_or_else(|_| unreachable!());

                let index = us / 4usize;
                let shift = (us % 4) * 2;

                let all_bits = vec[index];
                let unset = (0b11 & (vec[index] >> shift)) == 0b11;
                if unset {
                    number_solvable += 1;
                    let new_bits = all_bits & !(((!moves_mod_3) & 0b11) << shift);

                    vec[index] = new_bits;
                    //println!("Perm {perm:03?} set {index:03} shifted {shift:01} to {moves_mod_3:02b} -> {new_bits:08b}" );

                    for swaps in &move_swaps {
                        let next_perm = Self::combine_arrays(&perm_arr, swaps);
                        next.push(next_perm);
                    }
                }
            }

            std::mem::swap(current, next);
            moves_mod_3 += 1;
            moves_mod_3 %= 3;
        }

        Self {
            vec,
            moves,
            number_solvable: number_solvable
                .try_into()
                .unwrap_or_else(|_| unreachable!()),
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::SolveContext;
    use crate::{Inner, Permutation};

    fn head_swaps<I: Inner, const ELEMENTS: usize>(
    ) -> impl Iterator<Item = Permutation<I, ELEMENTS>> {
        (1..=ELEMENTS).map(|i| {
            let mut swaps = [0u8; ELEMENTS];
            swaps[0] = i as u8;

            Permutation::<I, ELEMENTS>::from_swaps(swaps.into_iter())
        })
    }

    fn test_solve<I: Inner, const ELEMENTS: usize>() {
        let moves = head_swaps().collect_vec();

        let context = SolveContext::<I, ELEMENTS>::new(moves);
        check_solutions(&context);
    }

    #[test]
    pub fn test_solve5() {
        test_solve::<u8, 5>();
    }
    #[test]
    pub fn test_solve6() {
        test_solve::<u16, 6>();
    }
    #[test]
    pub fn test_solve7() {
        test_solve::<u16, 7>();
    }
    #[test]
    pub fn test_solve8() {
        test_solve::<u16, 8>();
    }

    pub fn count_generated_solutions<I: Inner, const ELEMENTS: usize>() {
        let mut moves = vec![
            Permutation::<I, ELEMENTS>::reverse(),
            Permutation::<I, ELEMENTS>::rotate_left(),
            Permutation::<I, ELEMENTS>::rotate_right(),
            ];
        for n in 2..ELEMENTS {
            // moves.push(Permutation::<I, ELEMENTS>::rotate_n(n));
            moves.push(Permutation::<I, ELEMENTS>::interleave(n as u8));
        }


        moves.sort();
        moves.dedup();

        for m in moves.iter(){
            println!("{m}");
        }

        let context = SolveContext::<I, ELEMENTS>::new(moves);

        let moves_len = context.moves.len();

        let claimed_solvable = context
            .number_solvable
            .try_into()
            .unwrap_or_else(|_| panic!(""));
        let count_using_bits = count_solvable_bits(&context);
        assert_eq!(count_using_bits, claimed_solvable);
        println!(
            "{:08?} solvable of {:08?} with {moves_len} moves",
            context.number_solvable,
            I::get_factorial(ELEMENTS),
        );

        let total = check_solutions(&context);
        assert_eq!(total, claimed_solvable)
    }

    #[test]
    pub fn count_generated4() {
        count_generated_solutions::<u8, 4>();
    }

    #[test]
    pub fn count_generated5() {
        count_generated_solutions::<u8, 5>();
    }
    #[test]
    pub fn count_generated6() {
        count_generated_solutions::<u16, 6>();
    }
    #[test]
    pub fn count_generated7() {
        count_generated_solutions::<u16, 7>();
    }
    #[test]
    pub fn count_generated8() {
        count_generated_solutions::<u16, 8>();
    }

    fn check_solutions<I: Inner, const ELEMENTS: usize>(
        context: &SolveContext<I, ELEMENTS>,
    ) -> usize {
        let mut count = 0;
        for perm in Permutation::<I, ELEMENTS>::all() {
            if let Some(solution) = context.solve(perm) {


                let mut p1 = perm.clone();
                for x in solution.iter() {
                    p1 = p1.combine(&x);
                }
                //let len = solution.len();
                // println!(
                //     "{perm} solved in {len} steps with {}",
                //     solution.iter().join(", ")
                // );
                assert_eq!(Permutation::<I, ELEMENTS>::default(), p1);
                count += 1;
            }
        }
        count
    }

    fn count_solvable_bits<I: Inner, const ELEMENTS: usize>(
        context: &SolveContext<I, ELEMENTS>,
    ) -> usize {
        let mut count = 0;
        for bits in context.vec.iter() {
            for shift in [0, 2, 4, 6] {
                if bits >> shift & 0b11 != 0b11 {
                    count += 1;
                }
            }
        }
        count
    }

    // #[test]
    // pub fn test_solve9() {
    //     test_solve::<u32, 9>();
    // }
    // #[test]
    // pub fn test_solve10() {
    //     test_solve::<u32, 10>();
    // }
    // #[test]
    // pub fn test_solve11() {
    //     test_solve::<u32,11>();
    // }
    // #[test]
    // pub fn test_solve12() {
    //     test_solve::<u32, 12>();
    // }
}
