use std::any::type_name;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use importunate::*;

pub fn criterion_benchmark(c: &mut Criterion) {

    bench_apply::<u8, 5>(c);
    bench_apply::<u128, 5>(c);
    bench_apply::<u16, 8>(c);
    bench_apply::<u32, 12>(c);
    bench_apply::<u64, 15>(c);
    bench_apply::<u64, 20>(c);
    bench_apply::<u128, 34>(c);
    bench_new_index::<u128, 34>(c, 0);
    bench_new_index::<u128, 34>(c, 33);

    bench_old_index::<u128, 34>(c, 0);
    bench_old_index::<u128, 34>(c, 33);

    bench_calculate::<u8, 5>(c);
    bench_calculate::<u128, 5>(c);
    bench_calculate::<u16, 8>(c);
    bench_calculate::<u32, 12>(c);
    bench_calculate::<u64, 15>(c);
    bench_calculate::<u64, 20>(c);
    bench_calculate::<u128, 34>(c);


}

fn bench_old_index<Inner: PermutationInner, const SIZE: usize>(c: &mut Criterion, index: usize) {
    c.bench_function(format!("old index {SIZE} {index}").as_str(), |b| {
        let perm = Permutation::<Inner, SIZE>::get_max();

        b.iter(|| old_index(black_box(perm), index))
    });
}

fn bench_new_index<Inner: PermutationInner, const SIZE: usize>(c: &mut Criterion, index: usize) {
    c.bench_function(format!("new index {SIZE} {index}").as_str(), |b| {
        let perm = Permutation::<Inner, SIZE>::get_max();

        b.iter(|| new_index(black_box(perm), index))
    });
}

fn bench_apply<Inner: PermutationInner, const SIZE: usize>(c: &mut Criterion) {
    c.bench_function(
        format!("apply {} {SIZE}", type_name::<Inner>()).as_str(),
        |b| {
            let arr = Permutation::<Inner, SIZE>::get_max().get_array();
            let perm = Permutation::<Inner, SIZE>::try_calculate(arr, |&x|x).unwrap();
            let test_arr = arr;
            b.iter(|| apply(black_box(test_arr), perm))
        },
    );
}
fn bench_calculate<Inner: PermutationInner, const SIZE: usize>(c: &mut Criterion) {
    c.bench_function(
        format!("calculate {} {SIZE}", type_name::<Inner>()).as_str(),
        |b| {
            let mut arr: [usize; SIZE] = core::array::from_fn(|x| x);
            arr.reverse();
            let test_arr = arr;
            b.iter(|| calculate::<Inner, SIZE>(black_box(test_arr)))
        },
    );
}

fn calculate<Inner: PermutationInner, const SIZE: usize>(
    arr: [usize; SIZE],
) -> Permutation<u64, SIZE> {
    Permutation::try_calculate(arr, |&x|x).unwrap()
}

fn apply<Inner: PermutationInner, const SIZE: usize>(
    mut arr: [usize; SIZE],
    permutation: Permutation<Inner, SIZE>,
) -> [usize; SIZE] {
    permutation.apply(&mut arr);
    arr
}

fn new_index<Inner: PermutationInner, const SIZE: usize>(
    permutation: Permutation<Inner, SIZE>,
    index: usize,
) -> usize {
    permutation.index_of(&index, |&x|x)
}

fn old_index<Inner: PermutationInner, const SIZE: usize>(
    permutation: Permutation<Inner, SIZE>,
    index: usize,
) -> usize {
    permutation.element_at_index(index, |x|x)
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
