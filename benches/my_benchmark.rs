use std::any::type_name;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use importunate::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    bench_calculate::<u8, 5> (c);
    bench_calculate::<u128, 5> (c);
    bench_calculate::<u16, 8> (c);
    bench_calculate::<u32, 12> (c);
    bench_calculate::<u64, 15> (c);
    bench_calculate::<u64, 20> (c);
    bench_calculate::<u128, 34> (c);

    bench_apply::<u8, 5> (c);
    bench_calculate::<u128, 5> (c);
    bench_apply::<u16, 8> (c);
    bench_apply::<u32, 12> (c);
    bench_apply::<u64, 15> (c);
    bench_apply::<u64, 20> (c);
    bench_calculate::<u128, 34> (c);


    c.bench_function(format!("new index 20 19").as_str(), |b| {
        let mut arr: [usize; 20] = core::array::from_fn(|x| x);
        arr.reverse();
        let perm = Permutation::<u64, 20>::try_calculate(arr).unwrap();

        b.iter(|| new_index(black_box(perm), 19))
    });

    c.bench_function(format!("new index 20 0").as_str(), |b| {
        let mut arr: [usize; 20] = core::array::from_fn(|x| x);
        arr.reverse();
        let perm = Permutation::<u64, 20>::try_calculate(arr).unwrap();

        b.iter(|| new_index(black_box(perm), 0))
    });
}



fn bench_apply<Inner: PermutationInner, const SIZE: usize, >(c: &mut Criterion) {
    c.bench_function(format!("apply {} {SIZE}", type_name::<Inner>()).as_str(), |b| {
        let arr = Permutation::<Inner, SIZE>::DEFAULT_ARRAY;
        let perm = Permutation::<Inner, SIZE>::try_calculate(arr).unwrap();
        let test_arr = arr;
        b.iter(|| apply(black_box(test_arr), perm))
    });
}
fn bench_calculate< Inner: PermutationInner, const SIZE: usize,>(c: &mut Criterion) {
    c.bench_function(format!("calculate {} {SIZE}", type_name::<Inner>()).as_str(), |b| {
        let mut arr: [usize; SIZE] = core::array::from_fn(|x| x);
        arr.reverse();
        let test_arr = arr;
        b.iter(|| calculate::<Inner, SIZE, > (black_box(test_arr)))
    });
}

fn calculate<Inner: PermutationInner, const SIZE: usize, >(arr: [usize; SIZE]) -> Permutation<u64, SIZE> {
    Permutation::try_calculate(arr).unwrap()
}

fn apply<Inner: PermutationInner, const SIZE: usize, >(
    mut arr: [usize; SIZE],
    permutation: Permutation<Inner, SIZE>,
) -> [usize; SIZE] {
    permutation.apply(&mut arr);
    arr
}

fn new_index<const SIZE: usize>(permutation: Permutation<u64, SIZE>, index: usize) -> usize {
    permutation.get_new_index(index)
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
